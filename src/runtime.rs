use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use anyhow::{anyhow, Context, Result};

use crate::ast::{CmpOp, Cond, EchoTarget, GrainExpr, Item, Program, Step, Stmt};
use crate::geometry::{resonance, Grain, Space, RESONANCE_THRESHOLD};
use crate::parser;
use crate::speak;

pub fn run_source(source: &str) -> Result<String> {
    let program = parser::parse(source)?;
    let mut vm = Vm::new();
    vm.execute(&program)?;
    Ok(vm.output)
}

pub fn run_file(path: &Path) -> Result<String> {
    let source = fs::read_to_string(path)
        .with_context(|| format!("Could not read Trell source: {}", path.display()))?;
    run_source(&source)
}

struct Vm {
    space: Space,
    grains: HashMap<String, Grain>,
    paths: HashMap<String, Vec<Step>>,
    output: String,
}

impl Vm {
    fn new() -> Self {
        Self {
            space: Space::new(),
            grains: HashMap::new(),
            paths: HashMap::new(),
            output: String::new(),
        }
    }

    fn execute(&mut self, program: &Program) -> Result<()> {
        for item in &program.items {
            match item {
                Item::Axes(axes) => {
                    for axis in axes {
                        self.space.add_axis(
                            axis.name.clone(),
                            axis.low.clone(),
                            axis.high.clone(),
                        )?;
                    }
                }
                Item::Offer { coords, text } => {
                    self.space.add_offer(coords.clone(), text.clone())?;
                }
                Item::Path { name, steps } => {
                    self.paths.insert(name.clone(), steps.clone());
                }
                Item::Stmt(stmt) => self.exec_stmt(stmt)?,
            }
        }
        Ok(())
    }

    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Grain { name, expr } => {
                let grain = self.eval_grain(expr)?;
                self.grains.insert(name.clone(), grain);
            }
            Stmt::Speak(expr) => {
                let grain = self.eval_grain(expr)?;
                let spoken = speak::speak(&grain, &self.space);
                let header = format_header(&expr.label(), &grain, &self.space);
                self.output.push_str(&header);
                self.output.push('\n');
                self.output.push_str(&spoken);
                self.output.push_str("\n\n");
            }
            Stmt::Echo(target) => match target {
                EchoTarget::Space => {
                    self.output.push_str(&self.format_space());
                    self.output.push('\n');
                }
                EchoTarget::Grain(expr) => {
                    let grain = self.eval_grain(expr)?;
                    self.output
                        .push_str(&format_echo(&expr.label(), &grain, &self.space));
                    self.output.push('\n');
                }
            },
            Stmt::When {
                cond,
                then_body,
                else_body,
            } => {
                let body = if self.eval_cond(cond)? {
                    then_body
                } else {
                    else_body
                };
                for stmt in body {
                    self.exec_stmt(stmt)?;
                }
            }
        }
        Ok(())
    }

    fn eval_cond(&self, cond: &Cond) -> Result<bool> {
        match cond {
            Cond::ResonatePhrase { grain, phrase } => {
                let left = self.eval_grain(grain)?;
                let right = self.space.feel(phrase)?;
                Ok(resonance(&left, &right) >= RESONANCE_THRESHOLD)
            }
            Cond::ResonateGrains { left, right } => {
                let left = self.eval_grain(left)?;
                let right = self.eval_grain(right)?;
                Ok(resonance(&left, &right) >= RESONANCE_THRESHOLD)
            }
            Cond::Axis {
                grain,
                axis,
                op,
                value,
            } => {
                let grain = self
                    .grains
                    .get(grain)
                    .ok_or_else(|| anyhow!("Unknown grain '{grain}'"))?;
                let actual = grain.axis_value(&self.space, axis)?;
                let target = *value as f32;
                Ok(match op {
                    CmpOp::Gt => actual > target,
                    CmpOp::Lt => actual < target,
                    CmpOp::Ge => actual >= target,
                    CmpOp::Le => actual <= target,
                })
            }
        }
    }

    fn eval_grain(&self, expr: &GrainExpr) -> Result<Grain> {
        match expr {
            GrainExpr::Feel(text) => self.space.feel(text),
            GrainExpr::Name(name) => self
                .grains
                .get(name)
                .cloned()
                .ok_or_else(|| anyhow!("Unknown grain '{name}'")),
            GrainExpr::ShadowOf(inner) => {
                let grain = self.eval_grain(inner)?;
                grain
                    .shadow_grain()
                    .ok_or_else(|| anyhow!("Grain '{}' has no shadow", inner.label()))
            }
            GrainExpr::Blend { left, right, by } => {
                let left = self.eval_grain(left)?;
                let right = self.eval_grain(right)?;
                Ok(left.blend(&right, *by))
            }
            GrainExpr::Pipeline { base, steps } => {
                let grain = self.eval_grain(base)?;
                self.apply_steps(grain, steps, &HashSet::new())
            }
        }
    }

    fn apply_steps(
        &self,
        mut grain: Grain,
        steps: &[Step],
        inherited_frozen: &HashSet<String>,
    ) -> Result<Grain> {
        let mut frozen = inherited_frozen.clone();
        for step in steps {
            if let Step::Keeping(names) = step {
                for name in names {
                    self.space.axis_index(name)?;
                    frozen.insert(name.clone());
                }
            }
        }
        for step in steps {
            match step {
                Step::Keeping(_) => {}
                Step::Along { axis, toward, by } => {
                    grain = grain.along(&self.space, axis, *toward, by.unwrap_or(1.0), &frozen)?;
                }
                Step::Via(name) => {
                    let path = self
                        .paths
                        .get(name)
                        .cloned()
                        .ok_or_else(|| anyhow!("Unknown path '{name}'"))?;
                    grain = self.apply_steps(grain, &path, &frozen)?;
                }
                Step::Without(other) => {
                    let other = self.eval_grain(other)?;
                    grain = grain.without(&self.space, &other, &frozen);
                }
                Step::WithShadow => {
                    grain = grain.with_shadow();
                }
            }
        }
        Ok(grain)
    }

    fn format_space(&self) -> String {
        let mut out = String::from("space\n");
        for axis in &self.space.axes {
            out.push_str(&format!(
                "  {}  [{}] <-> [{}]\n",
                axis.name, axis.low, axis.high
            ));
        }
        for (index, offer) in self.space.offers.iter().enumerate() {
            out.push_str(&format!(
                "  offer {}  {}\n",
                index + 1,
                format_coords(&offer.coord, &self.space)
            ));
        }
        out
    }
}

fn format_header(label: &str, grain: &Grain, space: &Space) -> String {
    format!("— {label} · {} —", format_coords(&grain.coord, space))
}

fn format_echo(label: &str, grain: &Grain, space: &Space) -> String {
    let mut out = format!("{label}\n");
    for (index, axis) in space.axes.iter().enumerate() {
        out.push_str(&format!("  {:<12} {:.2}\n", axis.name, grain.coord[index]));
    }
    if let Some(shadow) = grain.shadow_grain() {
        out.push_str(&format!(
            "  shadow       {}\n",
            format_coords(&shadow.coord, space)
        ));
    } else {
        out.push_str("  shadow       none\n");
    }
    out
}

fn format_coords(coord: &[f32], space: &Space) -> String {
    space
        .axes
        .iter()
        .enumerate()
        .map(|(index, axis)| {
            format!(
                "{} {:.2}",
                axis.name,
                coord.get(index).copied().unwrap_or(0.5)
            )
        })
        .collect::<Vec<_>>()
        .join("  ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_a_tiny_program() {
        let source = r#"
            axes {
              warmth: "ice fluorescent chart" <-> "ember darling held"
            }
            offer at warmth=0.9:
              "I keep thinking of your hands."
            grain scene = feel "ice fluorescent chart"
            grain letter = scene along warmth toward 0.9
            speak letter
            speak shadow of letter
        "#;
        let output = run_source(source).unwrap();
        assert!(output.contains("hands"), "{output}");
        assert!(output.contains("shadow of letter"), "{output}");
    }
}
