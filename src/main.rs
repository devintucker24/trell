use anyhow::{anyhow, Context as AnyhowContext, Result};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use trell::codegen::compile_trell_package;
use trell::interpreter::{Interpreter, RuntimeValue};
use trell::lexer::lex;
use trell::llvm_backend;
use trell::oracle::ConfigurableOracle;
use trell::parser::Parser;
use trell::typecheck::TypeChecker;

fn print_banner() {
    println!("============================================================");
    println!("  TRELL: Epistemic Speculative Semantic Programming Engine  ");
    println!("============================================================");
}

fn print_usage() {
    println!("Usage: trell <command> [options] <source.trell>\n");
    println!("Commands:");
    println!("  run <file.trell> [--scenario <mock.json>]");
    println!(
        "      Typecheck, deliberate with oracles, and execute speculative semantic branches."
    );
    println!("  check <file.trell>");
    println!("      Epistemic type check: guarantees certain/belief soundness without execution.");
    println!("  inspect <file.trell>");
    println!("      Display AST, model contracts, guards, and epistemic boundaries.");
    println!("  compile <file.trell> [-o <output.trellc>]");
    println!("      Compile verified Trell program into an epistemic execution artifact.");
    println!("  emit-llvm <file.trell> [-o <output.ll>]");
    println!("      Lower the certain-integer core to LLVM IR (rejects epistemic constructs).");
    println!("  build <file.trell> [-o <output>]");
    println!("      Compile the certain-integer core to a native executable via LLVM.\n");
}

fn cmd_check(source_path: &Path) -> Result<()> {
    let source = fs::read_to_string(source_path).with_context(|| {
        format!(
            "Could not read Trell source file: {}",
            source_path.display()
        )
    })?;

    let tokens = lex(&source)?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;

    let mut checker = TypeChecker::new();
    checker.check_program(&program)?;

    println!(
        "SUCCESS: Epistemic type check passed for '{}'.",
        source_path.display()
    );
    println!("         Dual-track Certain/Belief boundaries are sound.");
    Ok(())
}

fn cmd_inspect(source_path: &Path) -> Result<()> {
    let source = fs::read_to_string(source_path).with_context(|| {
        format!(
            "Could not read Trell source file: {}",
            source_path.display()
        )
    })?;

    let tokens = lex(&source)?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;

    println!(
        "\n=== Trell Program Inspection: {} ===",
        source_path.display()
    );
    for item in &program.items {
        match item {
            trell::ast::Item::Contract(c) => {
                println!(
                    "[Contract] '{}' (model: {}, temp: {:?}, budget: {:?}, min_conf: {:?})",
                    c.name, c.model_kind, c.temperature, c.max_tokens, c.min_confidence
                );
            }
            trell::ast::Item::Struct(s) => {
                println!("[Struct] '{}' with {} fields", s.name, s.fields.len());
                for f in &s.fields {
                    println!("    - {}: {:?}", f.name, f.ty);
                }
            }
            trell::ast::Item::Guard(g) => {
                println!(
                    "[Guard] '{}' verifying ({}: {:?})",
                    g.name, g.param_name, g.param_type
                );
            }
            trell::ast::Item::Function(f) => {
                println!("[Function] '{}' -> {:?}", f.name, f.return_type);
                for p in &f.params {
                    println!("    param: {}: {:?}", p.name, p.ty);
                }
            }
        }
    }
    println!("====================================================\n");
    Ok(())
}

fn cmd_compile(source_path: &Path, output_path: Option<&Path>) -> Result<()> {
    let source = fs::read_to_string(source_path).with_context(|| {
        format!(
            "Could not read Trell source file: {}",
            source_path.display()
        )
    })?;

    let tokens = lex(&source)?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;

    let mut checker = TypeChecker::new();
    checker.check_program(&program)?;

    let out_buf = if let Some(p) = output_path {
        p.to_path_buf()
    } else {
        source_path.with_extension("trellc")
    };

    compile_trell_package(&program, source_path, &out_buf)?;
    println!(
        "Compiled '{}' -> '{}'",
        source_path.display(),
        out_buf.display()
    );
    Ok(())
}

fn typecheck_source(source_path: &Path) -> Result<trell::ast::Program> {
    let source = fs::read_to_string(source_path).with_context(|| {
        format!(
            "Could not read Trell source file: {}",
            source_path.display()
        )
    })?;

    let tokens = lex(&source)?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;

    let mut checker = TypeChecker::new();
    checker.check_program(&program)?;

    Ok(program)
}

fn cmd_emit_llvm(source_path: &Path, output_path: Option<&Path>) -> Result<()> {
    let program = typecheck_source(source_path)?;

    let ir = llvm_backend::compile_program_to_ir(&program)?;

    let out_buf = match output_path {
        Some(p) => p.to_path_buf(),
        None => source_path.with_extension("ll"),
    };
    fs::write(&out_buf, ir)
        .with_context(|| format!("Could not write LLVM IR to {}", out_buf.display()))?;

    println!(
        "Lowered certain-integer core of '{}' -> '{}'",
        source_path.display(),
        out_buf.display()
    );
    Ok(())
}

fn cmd_build(source_path: &Path, output_path: Option<&Path>) -> Result<()> {
    let program = typecheck_source(source_path)?;

    let out_buf = match output_path {
        Some(p) => p.to_path_buf(),
        None => source_path.with_extension(""),
    };
    let object_path = out_buf.with_extension("o");

    llvm_backend::compile_program_to_object(&program, &object_path)?;

    // Link the object into a native executable using the system C toolchain.
    let status = Command::new("cc")
        .arg(&object_path)
        .arg("-o")
        .arg(&out_buf)
        .status()
        .with_context(|| "Could not invoke the C linker `cc` to produce the executable")?;
    if !status.success() {
        return Err(anyhow!(
            "linking failed while producing '{}' (cc exited with {})",
            out_buf.display(),
            status
        ));
    }
    let _ = fs::remove_file(&object_path);

    println!(
        "Compiled certain-integer core of '{}' -> native executable '{}'",
        source_path.display(),
        out_buf.display()
    );
    println!(
        "Run it and inspect its exit code, e.g.: {} ; echo $?",
        out_buf.display()
    );
    Ok(())
}

fn cmd_run(source_path: &Path, scenario_file: Option<&Path>) -> Result<()> {
    let source = fs::read_to_string(source_path).with_context(|| {
        format!(
            "Could not read Trell source file: {}",
            source_path.display()
        )
    })?;

    // Step 1: Lex & Parse
    let tokens = lex(&source)?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;

    // Step 2: Epistemic Type Check
    let mut checker = TypeChecker::new();
    checker.check_program(&program)?;

    // Step 3: Setup Oracle
    let oracle = if let Some(path) = scenario_file {
        ConfigurableOracle::load_from_file(path)?
    } else {
        ConfigurableOracle::new()
    };

    // Step 4: Run Interpreter
    let mut interpreter = Interpreter::new(&program, Box::new(oracle));
    let exit_val = interpreter.run_main()?;

    // Step 5: Report Speculative Execution Collapses
    if !interpreter.traces.is_empty() {
        println!("\n[Speculative Semantic Execution Report]");
        for (idx, trace) in interpreter.traces.iter().enumerate() {
            println!("  Fork #{}:", idx + 1);
            println!("    Target semantic state: \"{}\"", trace.target_value);
            println!("    Committed branch:      \"{}\"", trace.chosen_branch);
            if !trace.rolled_back_branches.is_empty() {
                println!(
                    "    Rolled back branches:  {:?}",
                    trace.rolled_back_branches
                );
            }
        }
        println!();
    }

    if exit_val != RuntimeValue::Unit {
        println!("[Program Final Result]: {}", exit_val);
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_banner();
        print_usage();
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "check" => {
            if args.len() < 3 {
                return Err(anyhow!("Usage: trell check <file.trell>"));
            }
            cmd_check(Path::new(&args[2]))?;
        }
        "inspect" => {
            if args.len() < 3 {
                return Err(anyhow!("Usage: trell inspect <file.trell>"));
            }
            cmd_inspect(Path::new(&args[2]))?;
        }
        "compile" => {
            if args.len() < 3 {
                return Err(anyhow!(
                    "Usage: trell compile <file.trell> [-o <out.trellc>]"
                ));
            }
            let src = Path::new(&args[2]);
            let out = if args.len() >= 5 && args[3] == "-o" {
                Some(Path::new(&args[4]))
            } else {
                None
            };
            cmd_compile(src, out)?;
        }
        "emit-llvm" => {
            if args.len() < 3 {
                return Err(anyhow!("Usage: trell emit-llvm <file.trell> [-o <out.ll>]"));
            }
            let src = Path::new(&args[2]);
            let out = if args.len() >= 5 && args[3] == "-o" {
                Some(Path::new(&args[4]))
            } else {
                None
            };
            cmd_emit_llvm(src, out)?;
        }
        "build" => {
            if args.len() < 3 {
                return Err(anyhow!("Usage: trell build <file.trell> [-o <output>]"));
            }
            let src = Path::new(&args[2]);
            let out = if args.len() >= 5 && args[3] == "-o" {
                Some(Path::new(&args[4]))
            } else {
                None
            };
            cmd_build(src, out)?;
        }
        "run" => {
            if args.len() < 3 {
                return Err(anyhow!(
                    "Usage: trell run <file.trell> [--scenario <mock.json>]"
                ));
            }
            let src = Path::new(&args[2]);
            let scenario = if args.len() >= 5 && args[3] == "--scenario" {
                Some(Path::new(&args[4]))
            } else {
                None
            };
            cmd_run(src, scenario)?;
        }
        // Direct file pass (backwards compatibility with trell <file.trell>)
        file if file.ends_with(".trell") => {
            cmd_run(Path::new(file), None)?;
        }
        _ => {
            print_usage();
            return Err(anyhow!("Unknown command '{}'", command));
        }
    }

    Ok(())
}
