use anyhow::{anyhow, Result};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::IntValue;

use crate::ast::{BinaryOperator, Expr};

pub fn codegen_expr<'ctx>(
    context: &'ctx Context,
    builder: &Builder<'ctx>,
    expression: &Expr,
) -> Result<IntValue<'ctx>> {
    let i64_type = context.i64_type();

    match expression {
        Expr::Integer(value) => Ok(i64_type.const_int(*value, false)),

        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let left_value = codegen_expr(context, builder, left)?;
            let right_value = codegen_expr(context, builder, right)?;

            let value = match operator {
                BinaryOperator::Add => builder
                    .build_int_add(left_value, right_value, "addtmp")
                    .map_err(|error| anyhow!("Could not generate addition: {error:?}"))?,

                BinaryOperator::Subtract => builder
                    .build_int_sub(left_value, right_value, "subtmp")
                    .map_err(|error| anyhow!("Could not generate subtraction: {error:?}"))?,

                BinaryOperator::Multiply => builder
                    .build_int_mul(left_value, right_value, "multmp")
                    .map_err(|error| anyhow!("Could not generate multiplication: {error:?}"))?,

                BinaryOperator::Divide => builder
                    .build_int_unsigned_div(left_value, right_value, "divtmp")
                    .map_err(|error| anyhow!("Could not generate division: {error:?}"))?,
            };

            Ok(value)
        }
    }
}
