mod ast;
mod codegen;
mod lexer;
mod parser;

use anyhow::{anyhow, Context as AnyhowContext, Result};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::TargetMachine;
use inkwell::values::FunctionValue;
use std::env;
use std::fs;
use std::path::Path;

use ast::Expr;
use codegen::codegen_expr;
use lexer::lex;
use parser::Parser;

fn build_main<'ctx>(
    context: &'ctx Context,
    module: &Module<'ctx>,
    expression: &Expr,
) -> Result<FunctionValue<'ctx>> {
    let i64_type = context.i64_type();
    let main_type = i64_type.fn_type(&[], false);
    let main = module.add_function("main", main_type, None);

    let entry = context.append_basic_block(main, "entry");
    let builder = context.create_builder();

    builder.position_at_end(entry);

    let result = codegen_expr(context, &builder, expression)?;

    builder
        .build_return(Some(&result))
        .map_err(|error| anyhow!("Could not generate return instruction: {error:?}"))?;

    Ok(main)
}

fn compile(source_path: &Path) -> Result<()> {
    let source = fs::read_to_string(source_path).with_context(|| {
        format!(
            "Could not read Trell source file: {}",
            source_path.display()
        )
    })?;

    let tokens = lex(&source)?;
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_program()?;

    let context = Context::create();
    let module = context.create_module("trell");
    module.set_triple(&TargetMachine::get_default_triple());

    let main_fn = build_main(&context, &module, &expression)?;

    if !main_fn.verify(true) {
        return Err(anyhow!("LLVM function verification failed"));
    }

    module
        .verify()
        .map_err(|error| anyhow!("LLVM module verification failed: {error}"))?;

    module
        .print_to_file("out.ll")
        .map_err(|error| anyhow!("Could not write out.ll: {error}"))?;

    println!("Compiled {} → out.ll", source_path.display());

    Ok(())
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);

    let source_path = args
        .next()
        .ok_or_else(|| anyhow!("Usage: trell <source-file.trell>"))?;

    if args.next().is_some() {
        return Err(anyhow!("Usage: trell <source-file.trell>"));
    }

    compile(Path::new(&source_path))
}
