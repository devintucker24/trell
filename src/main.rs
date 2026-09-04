use anyhow::{anyhow, bail, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use trell::check::CheckedProgram;
use trell::compile_source;
use trell::diagnostics::{eprint_all, render_all};
use trell::gbnf;
use trell::interp::{self, Host};
use trell::plan;
use trell::value::Value;
use trell::wasm;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() || args[0] == "-h" || args[0] == "--help" {
        print_help();
        return Ok(());
    }

    let json = take_flag(&mut args, "--json");
    match args[0].as_str() {
        "check" => {
            let path = require_file(&args[1..], "trell check <file>")?;
            let (source, checked) = load(&path)?;
            if json {
                print_check_json(&checked);
            } else {
                println!("ok  {}", path.display());
                if !checked.warnings.is_empty() {
                    eprint!(
                        "{}",
                        render_all(&path_str(&path), &source, &checked.warnings)
                    );
                }
            }
            Ok(())
        }
        "plan" => {
            let path = require_file(&args[1..], "trell plan <file>")?;
            let (_source, checked) = load(&path)?;
            let planned = plan::plan(&checked);
            if json {
                println!("{}", serde_json::to_string_pretty(&planned)?);
            } else {
                print!("{}", plan::render(&planned));
            }
            Ok(())
        }
        "run" => {
            let (path, host) = parse_run_args(&args[1..])?;
            let (source, checked) = load(&path)?;
            match interp::run(&checked, host) {
                Ok(result) => {
                    if json {
                        let payload = serde_json::json!({
                            "value": result.value,
                            "sends": result.sends,
                            "writes": result.writes,
                            "approvals": result.approvals,
                            "asks": result.asks,
                        });
                        println!("{}", serde_json::to_string_pretty(&payload)?);
                    } else {
                        if !result.approvals.is_empty() {
                            for message in &result.approvals {
                                println!("approve  {message}");
                            }
                        }
                        for (path, body) in &result.writes {
                            println!("write    {path}");
                            println!("{body}");
                        }
                        for send in &result.sends {
                            println!("send     {send}");
                        }
                        if !matches!(result.value, Value::Unit) {
                            println!("{}", result.value);
                        }
                    }
                    Ok(())
                }
                Err(diagnostic) => {
                    eprint_all(&path_str(&path), &source, &[diagnostic])?;
                    bail!("run failed");
                }
            }
        }
        "eval" => {
            let (path, host) = parse_run_args(&args[1..])?;
            let (source, checked) = load(&path)?;
            if checked.program.is_pure_compute() {
                let module = wasm::compile(&checked.program).map_err(|d| {
                    eprint!("{}", render_all(&path_str(&path), &source, &[d]));
                    anyhow!("compile failed")
                })?;
                let mut args_i64 = Vec::new();
                for input in &checked.program.inputs {
                    match host.inputs.get(&input.name.name) {
                        Some(Value::Int(v)) => args_i64.push(*v),
                        Some(other) => bail!(
                            "input `{}` must be int for Wasm eval, got {}",
                            input.name.name,
                            other.type_name()
                        ),
                        None => args_i64.push(0),
                    }
                }
                let value =
                    wasm::interpret(&module, &args_i64).map_err(|e| anyhow!("Wasm trap: {e}"))?;
                println!("{value}");
            } else {
                let result = interp::run(&checked, host).map_err(|d| {
                    eprint!("{}", render_all(&path_str(&path), &source, &[d]));
                    anyhow!("eval failed")
                })?;
                println!("{}", result.value);
            }
            Ok(())
        }
        "compile" => {
            let (path, output, wat) = parse_compile_args(&args[1..])?;
            let (source, checked) = load(&path)?;
            let module = wasm::compile(&checked.program).map_err(|d| {
                eprint!("{}", render_all(&path_str(&path), &source, &[d]));
                anyhow!("compile failed")
            })?;
            if wat {
                let text = wasm::to_wat(&module);
                if let Some(output) = output {
                    fs::write(&output, text)?;
                    println!("Compiled {} → {}", path.display(), output.display());
                } else {
                    print!("{text}");
                }
            } else {
                let bytes = wasm::encode_wasm(&module);
                let output = output.unwrap_or_else(|| PathBuf::from("out.wasm"));
                fs::write(&output, bytes)?;
                println!(
                    "Compiled {} → {} (import-free Wasm, export eval)",
                    path.display(),
                    output.display()
                );
            }
            Ok(())
        }
        "grammar" => {
            print!("{}", gbnf::gbnf());
            Ok(())
        }
        other if other.starts_with('-') => {
            print_help();
            bail!("Unknown flag: {other}");
        }
        _ => {
            // Shorthand: check, then eval compute or print a plan.
            let path = PathBuf::from(&args[0]);
            let (source, checked) = load(&path)?;
            if checked.program.is_pure_compute() {
                let module = wasm::compile(&checked.program).map_err(|d| {
                    eprint!("{}", render_all(&path_str(&path), &source, &[d]));
                    anyhow!("compile failed")
                })?;
                let value = wasm::interpret(&module, &[]).map_err(|e| anyhow!("Wasm trap: {e}"))?;
                println!("{value}");
            } else {
                print!("{}", plan::render(&plan::plan(&checked)));
            }
            Ok(())
        }
    }
}

fn load(path: &Path) -> Result<(String, CheckedProgram)> {
    let source =
        fs::read_to_string(path).map_err(|e| anyhow!("Could not read {}: {e}", path.display()))?;
    match compile_source(&source) {
        Ok(checked) => Ok((source, checked)),
        Err(diagnostics) => {
            eprint_all(&path_str(path), &source, &diagnostics)?;
            bail!("check failed");
        }
    }
}

fn require_file(args: &[String], usage: &str) -> Result<PathBuf> {
    if args.len() != 1 {
        bail!("Usage: {usage}");
    }
    Ok(PathBuf::from(&args[0]))
}

fn parse_run_args(args: &[String]) -> Result<(PathBuf, Host)> {
    let mut path = None;
    let mut host = Host::default();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--mock" => {
                // default host is already a mock
            }
            "--auto-approve" => host.auto_approve = true,
            "--no-approve" => host.auto_approve = false,
            "--set" => {
                i += 1;
                if i >= args.len() {
                    bail!("--set requires name=value");
                }
                let (name, value) = parse_set(&args[i])?;
                host.inputs.insert(name, value);
            }
            "--ask" => {
                i += 1;
                if i >= args.len() {
                    bail!("--ask requires JSON");
                }
                let value: Value = serde_json::from_str(&args[i])
                    .map_err(|e| anyhow!("invalid --ask JSON: {e}"))?;
                host.ask_replies.push(value);
            }
            flag if flag.starts_with("--set=") => {
                let (name, value) = parse_set(&flag["--set=".len()..])?;
                host.inputs.insert(name, value);
            }
            other if other.starts_with('-') => bail!("Unknown flag: {other}"),
            other => {
                if path.is_some() {
                    bail!("Usage: trell run <file> [--set name=value] [--ask JSON]");
                }
                path = Some(PathBuf::from(other));
            }
        }
        i += 1;
    }
    let path = path.ok_or_else(|| anyhow!("Usage: trell run <file> [--set name=value]"))?;
    Ok((path, host))
}

fn parse_compile_args(args: &[String]) -> Result<(PathBuf, Option<PathBuf>, bool)> {
    let mut path = None;
    let mut output = None;
    let mut wat = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--wat" => wat = true,
            "-o" | "--output" => {
                i += 1;
                if i >= args.len() {
                    bail!("-o requires a path");
                }
                output = Some(PathBuf::from(&args[i]));
            }
            other if other.starts_with('-') => bail!("Unknown flag: {other}"),
            other => {
                if path.is_some() {
                    bail!("Usage: trell compile <file> [-o out.wasm] [--wat]");
                }
                path = Some(PathBuf::from(other));
            }
        }
        i += 1;
    }
    Ok((
        path.ok_or_else(|| anyhow!("Usage: trell compile <file> [-o out.wasm] [--wat]"))?,
        output,
        wat,
    ))
}

fn parse_set(spec: &str) -> Result<(String, Value)> {
    let (name, raw) = spec
        .split_once('=')
        .ok_or_else(|| anyhow!("--set requires name=value"))?;
    let value = if raw == "true" {
        Value::Bool(true)
    } else if raw == "false" {
        Value::Bool(false)
    } else if let Ok(n) = raw.parse::<i64>() {
        Value::Int(n)
    } else {
        Value::Text(raw.to_string())
    };
    Ok((name.to_string(), value))
}

fn take_flag(args: &mut Vec<String>, flag: &str) -> bool {
    let present = args.iter().any(|a| a == flag);
    args.retain(|a| a != flag);
    present
}

fn path_str(path: &Path) -> String {
    path.display().to_string()
}

fn print_check_json(checked: &CheckedProgram) {
    let payload = serde_json::json!({
        "ok": true,
        "name": checked.caps.name,
        "allowed": checked.caps.allowed,
        "denied": checked.caps.denied,
        "budget_tokens": checked.caps.budget_tokens,
        "budget_cents": checked.caps.budget_cents,
        "spawn_limit": checked.caps.spawn_limit,
        "need_approve": checked.caps.need_approve,
        "effects": checked.effects,
        "pure_compute": checked.program.is_pure_compute(),
    });
    println!("{}", serde_json::to_string_pretty(&payload).unwrap());
}

fn print_help() {
    print!(
        "\
trell — capability-checked programs for untrusted authors

Usage:
  trell <file>                  check, then eval (compute) or plan (workflow)
  trell check <file> [--json]   fail closed: types, capabilities, taint
  trell plan  <file> [--json]   show grants, budgets, effects (terraform-plan)
  trell run   <file> [flags]    mock-execute a workflow
  trell eval  <file> [flags]    evaluate; pure compute runs in a Wasm sandbox
  trell compile <file> [-o out.wasm] [--wat]
  trell grammar                 print GBNF for constrained decoding

Run flags:
  --set name=value              bind an `in` parameter
  --ask '{{...}}'               mock the next `ask` reply as JSON
  --no-approve                  stop at `approve` instead of auto-passing

Trell is not a Python agent framework. It is the file you review in git:
who may ask, which tools exist, whether a child may spawn, whether a human
must approve — refused before anything spends.
"
    );
}
