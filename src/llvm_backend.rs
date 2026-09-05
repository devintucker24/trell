//! Native LLVM backend for the *certain integer* core of Trell.
//!
//! Trell's thesis is that a `belief<T>` is not a `certain T`: beliefs are only
//! reduced to certainties through `verify`/`require` + `guard`, and speculative
//! `fork`s need the belief runtime to track and roll back branches. None of that
//! has a meaningful, sound lowering to a straight native instruction stream.
//!
//! This backend therefore compiles only the fully-grounded, deterministic subset
//! of the language — integer literals, integer arithmetic, `let`/assignment,
//! function calls, and `return` — into LLVM IR (and, from there, native objects
//! and JIT execution). Every epistemic construct (`oracle`, `verify`,
//! `consensus`, `fork`, `confidence`, `justification`) is *rejected* with a clear
//! error rather than silently lowered, preserving the certain/belief boundary.
//!
//! Entry function requirements: a program must define `fn main() -> int` taking
//! no parameters. Its returned integer becomes the process exit code, matching
//! the classic `llc`/`cc` native pipeline.

use std::collections::HashMap;
use std::path::Path;

use anyhow::{anyhow, Result};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicMetadataValueEnum, FunctionValue, IntValue};
use inkwell::OptimizationLevel;

use crate::ast::{BinaryOp, Expr, FunctionDef, Item, Literal, PrimitiveType, Program, Stmt, Type};

/// A function is native-compilable when every parameter and its return type are
/// `certain int`. Anything touching beliefs, floats, bools, strings or structs
/// is out of scope for this integer-only backend.
fn is_native_compilable(function: &FunctionDef) -> bool {
    let is_certain_int = |ty: &Type| matches!(ty, Type::Certain(PrimitiveType::Int));
    is_certain_int(&function.return_type) && function.params.iter().all(|p| is_certain_int(&p.ty))
}

/// Compile a whole program into an LLVM IR string.
pub fn compile_program_to_ir(program: &Program) -> Result<String> {
    let context = Context::create();
    let module = build_module(&context, program)?;
    Ok(module.print_to_string().to_string())
}

/// JIT-compile and execute `main`, returning its integer result. Used both by
/// `trell` at runtime-free test time and by the test suite to prove that the
/// generated code actually computes the right values.
pub fn jit_run_main(program: &Program) -> Result<i64> {
    Target::initialize_native(&InitializationConfig::default())
        .map_err(|e| anyhow!("failed to initialize native target: {e}"))?;

    let context = Context::create();
    let module = build_module(&context, program)?;

    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| anyhow!("failed to create JIT execution engine: {e}"))?;

    // Safety: `main` is generated with the C ABI `fn() -> i64` signature by
    // `build_module`, and the module was verified before we get here.
    let main = unsafe {
        execution_engine
            .get_function::<unsafe extern "C" fn() -> i64>("main")
            .map_err(|e| anyhow!("no native-compilable `main` to execute: {e}"))?
    };

    Ok(unsafe { main.call() })
}

/// Emit a native object file for `program` to `path`.
pub fn compile_program_to_object(program: &Program, path: &Path) -> Result<()> {
    Target::initialize_native(&InitializationConfig::default())
        .map_err(|e| anyhow!("failed to initialize native target: {e}"))?;

    let context = Context::create();
    let module = build_module(&context, program)?;

    let triple = TargetMachine::get_default_triple();
    let target =
        Target::from_triple(&triple).map_err(|e| anyhow!("could not resolve target: {e:?}"))?;
    let target_machine = target
        .create_target_machine(
            &triple,
            "generic",
            "",
            OptimizationLevel::None,
            RelocMode::PIC,
            CodeModel::Default,
        )
        .ok_or_else(|| anyhow!("could not create target machine for {}", triple))?;

    module.set_triple(&triple);
    module.set_data_layout(&target_machine.get_target_data().get_data_layout());

    target_machine
        .write_to_file(&module, FileType::Object, path)
        .map_err(|e| anyhow!("could not write object file: {e:?}"))?;

    Ok(())
}

fn build_module<'ctx>(context: &'ctx Context, program: &Program) -> Result<Module<'ctx>> {
    let module = context.create_module("trell");
    let builder = context.create_builder();
    let i64_type = context.i64_type();

    // Pass 1: declare every native-compilable function so calls can resolve
    // regardless of definition order.
    let mut functions: HashMap<String, FunctionValue<'ctx>> = HashMap::new();
    let mut compilable: Vec<&FunctionDef> = Vec::new();
    for item in &program.items {
        if let Item::Function(function) = item {
            if is_native_compilable(function) {
                let param_types: Vec<BasicMetadataTypeEnum> =
                    function.params.iter().map(|_| i64_type.into()).collect();
                let fn_type = i64_type.fn_type(&param_types, false);
                let value = module.add_function(&function.name, fn_type, None);
                functions.insert(function.name.clone(), value);
                compilable.push(function);
            }
        }
    }

    // Require a native-compilable entry point.
    let main = program
        .items
        .iter()
        .find_map(|item| match item {
            Item::Function(f) if f.name == "main" => Some(f),
            _ => None,
        })
        .ok_or_else(|| anyhow!("native backend: program has no `main` function"))?;
    if !main.params.is_empty() || !is_native_compilable(main) {
        return Err(anyhow!(
            "native backend: entry point must be `fn main() -> int` with no parameters \
             (beliefs and other types must stay in the interpreted `trell run` pipeline)"
        ));
    }

    // Pass 2: generate each function body.
    for function in compilable {
        let value = functions[&function.name];
        let mut codegen = FunctionCodegen {
            context,
            builder: &builder,
            functions: &functions,
            vars: HashMap::new(),
        };
        codegen.emit(function, value)?;
    }

    module
        .verify()
        .map_err(|e| anyhow!("LLVM module verification failed: {e}"))?;

    Ok(module)
}

struct FunctionCodegen<'a, 'ctx> {
    context: &'ctx Context,
    builder: &'a Builder<'ctx>,
    functions: &'a HashMap<String, FunctionValue<'ctx>>,
    vars: HashMap<String, IntValue<'ctx>>,
}

impl<'ctx> FunctionCodegen<'_, 'ctx> {
    fn emit(&mut self, function: &FunctionDef, value: FunctionValue<'ctx>) -> Result<()> {
        let entry = self.context.append_basic_block(value, "entry");
        self.builder.position_at_end(entry);

        for (index, param) in function.params.iter().enumerate() {
            let arg = value
                .get_nth_param(index as u32)
                .ok_or_else(|| anyhow!("native backend: missing parameter {index}"))?
                .into_int_value();
            arg.set_name(&param.name);
            self.vars.insert(param.name.clone(), arg);
        }

        let last = function.body.len().wrapping_sub(1);
        let mut returned = false;
        for (index, stmt) in function.body.iter().enumerate() {
            match stmt {
                Stmt::Let { name, value, .. } => {
                    let v = self.expr(value)?;
                    self.vars.insert(name.clone(), v);
                }
                Stmt::Assign { target, value } => {
                    let v = self.expr(value)?;
                    self.vars.insert(target.clone(), v);
                }
                Stmt::Return(Some(expr)) => {
                    let v = self.expr(expr)?;
                    self.build_return(v)?;
                    returned = true;
                    break;
                }
                Stmt::Return(None) => {
                    return Err(anyhow!(
                        "native backend: bare `return;` is not valid in the int function '{}'",
                        function.name
                    ));
                }
                // A trailing expression statement is the implicit return value,
                // mirroring the interpreter's function semantics.
                Stmt::Expr(expr) if index == last => {
                    let v = self.expr(expr)?;
                    self.build_return(v)?;
                    returned = true;
                }
                Stmt::Expr(expr) => {
                    // Pure integer expressions have no side effects, but we still
                    // lower them so unsupported constructs are reported.
                    let _ = self.expr(expr)?;
                }
                Stmt::Print(_) | Stmt::Assert { .. } => {
                    return Err(anyhow!(
                        "native backend: `print`/`assert` are not supported; the integer-only \
                         backend compiles pure computation. Use `trell run` for I/O and invariants"
                    ));
                }
            }
        }

        if !returned {
            return Err(anyhow!(
                "native backend: function '{}' must end by returning an int \
                 (add a `return <int>;` or a trailing integer expression)",
                function.name
            ));
        }

        Ok(())
    }

    fn build_return(&self, value: IntValue<'ctx>) -> Result<()> {
        self.builder
            .build_return(Some(&value))
            .map_err(|e| anyhow!("LLVM build error (return): {e:?}"))?;
        Ok(())
    }

    fn expr(&mut self, expr: &Expr) -> Result<IntValue<'ctx>> {
        match expr {
            Expr::Lit(Literal::Int(n)) => Ok(self.context.i64_type().const_int(*n as u64, true)),
            Expr::Lit(other) => Err(anyhow!(
                "native backend: only integer literals compile to native code, found {other:?}"
            )),
            Expr::Ident(name) => self
                .vars
                .get(name)
                .copied()
                .ok_or_else(|| anyhow!("native backend: undefined variable '{name}'")),
            Expr::Binary { left, op, right } => {
                let l = self.expr(left)?;
                let r = self.expr(right)?;
                let result = match op {
                    BinaryOp::Add => self.builder.build_int_add(l, r, "addtmp"),
                    BinaryOp::Sub => self.builder.build_int_sub(l, r, "subtmp"),
                    BinaryOp::Mul => self.builder.build_int_mul(l, r, "multmp"),
                    BinaryOp::Div => self.builder.build_int_signed_div(l, r, "divtmp"),
                    BinaryOp::Mod => self.builder.build_int_signed_rem(l, r, "modtmp"),
                    other => {
                        return Err(anyhow!(
                            "native backend: operator {other:?} yields a non-integer \
                             (comparison/logical ops are not supported by the integer backend)"
                        ))
                    }
                }
                .map_err(|e| anyhow!("LLVM build error: {e:?}"))?;
                Ok(result)
            }
            Expr::Block(stmts, tail) => {
                for stmt in stmts {
                    match stmt {
                        Stmt::Let { name, value, .. } => {
                            let v = self.expr(value)?;
                            self.vars.insert(name.clone(), v);
                        }
                        Stmt::Assign { target, value } => {
                            let v = self.expr(value)?;
                            self.vars.insert(target.clone(), v);
                        }
                        Stmt::Expr(inner) => {
                            let _ = self.expr(inner)?;
                        }
                        _ => {
                            return Err(anyhow!(
                                "native backend: unsupported statement inside block expression"
                            ))
                        }
                    }
                }
                match tail {
                    Some(tail) => self.expr(tail),
                    None => Err(anyhow!(
                        "native backend: block expression has no trailing integer value"
                    )),
                }
            }
            Expr::Call { function, args } => {
                let callee = self.functions.get(function).copied().ok_or_else(|| {
                    anyhow!(
                        "native backend: call to '{function}' which is not a native-compilable \
                         `int(int, ...) -> int` function"
                    )
                })?;
                let mut compiled: Vec<BasicMetadataValueEnum> = Vec::with_capacity(args.len());
                for arg in args {
                    compiled.push(self.expr(arg)?.into());
                }
                let call = self
                    .builder
                    .build_call(callee, &compiled, "calltmp")
                    .map_err(|e| anyhow!("LLVM build error (call): {e:?}"))?;
                let result = call
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| anyhow!("native backend: '{function}' returned no value"))?
                    .into_int_value();
                Ok(result)
            }
            Expr::UnaryNot(_) => Err(anyhow!(
                "native backend: boolean `not` is not supported by the integer backend"
            )),
            Expr::FieldAccess { .. } | Expr::StructInit { .. } => Err(anyhow!(
                "native backend: struct values are not supported by the integer backend"
            )),
            Expr::OracleCall { .. }
            | Expr::Verify { .. }
            | Expr::Consensus { .. }
            | Expr::Fork { .. }
            | Expr::Confidence(_)
            | Expr::Justification(_) => Err(anyhow!(
                "native backend: epistemic constructs (oracle/verify/consensus/fork/confidence/\
                 justification) cannot be lowered to native code — a belief is not a certainty. \
                 Run these with `trell run`"
            )),
        }
    }
}
