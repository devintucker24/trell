use crate::check::{Caps, CheckedProgram, EffectUse};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Plan {
    pub name: Option<String>,
    pub allowed: Vec<Grant>,
    pub denied: Vec<String>,
    pub budgets: Budgets,
    pub spawn_limit: i64,
    pub gates: Vec<String>,
    pub effects: Vec<EffectUse>,
    pub inputs: Vec<String>,
    pub pure_compute: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Grant {
    pub effect: String,
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Budgets {
    pub tokens: i64,
    pub cents: i64,
}

pub fn plan(checked: &CheckedProgram) -> Plan {
    let caps: &Caps = &checked.caps;
    Plan {
        name: caps.name.clone(),
        allowed: caps
            .allowed
            .iter()
            .map(|(effect, paths)| Grant {
                effect: effect.clone(),
                paths: paths.clone(),
            })
            .collect(),
        denied: caps.denied.iter().cloned().collect(),
        budgets: Budgets {
            tokens: caps.budget_tokens,
            cents: caps.budget_cents,
        },
        spawn_limit: caps.spawn_limit,
        gates: caps.need_approve.iter().cloned().collect(),
        effects: checked.effects.clone(),
        inputs: checked
            .program
            .inputs
            .iter()
            .map(|i| format!("{}: {}", i.name.name, i.ty.name()))
            .collect(),
        pure_compute: checked.program.is_pure_compute(),
    }
}

pub fn render(plan: &Plan) -> String {
    let mut out = String::new();
    out.push_str("Trell plan\n");
    out.push_str("==========\n\n");

    if let Some(name) = &plan.name {
        out.push_str(&format!("Program  {name}\n\n"));
    }

    if plan.pure_compute {
        out.push_str("Kind     pure compute (no ambient authority)\n");
        out.push_str("Target   import-free Wasm\n\n");
    }

    out.push_str("Capabilities\n");
    if plan.allowed.is_empty() && plan.denied.is_empty() {
        out.push_str("  (none — fail closed; only pure compute is legal)\n");
    }
    for grant in &plan.allowed {
        if grant.paths.is_empty() {
            out.push_str(&format!("  allow  {}\n", grant.effect));
        } else {
            out.push_str(&format!(
                "  allow  {}  {}\n",
                grant.effect,
                grant.paths.join(" ")
            ));
        }
    }
    for denied in &plan.denied {
        out.push_str(&format!("  deny   {denied}\n"));
    }
    out.push_str(&format!(
        "  budget {} tokens, {} cents\n",
        plan.budgets.tokens, plan.budgets.cents
    ));
    out.push_str(&format!("  spawn  {}\n", plan.spawn_limit));
    for gate in &plan.gates {
        out.push_str(&format!("  gate   {gate} → approve\n"));
    }

    if !plan.inputs.is_empty() {
        out.push_str("\nInputs\n");
        for input in &plan.inputs {
            out.push_str(&format!("  {input}\n"));
        }
    }

    out.push_str("\nEffects\n");
    if plan.effects.is_empty() {
        out.push_str("  (none)\n");
    }
    for effect in &plan.effects {
        let taint = if effect.tainted { "  tainted" } else { "" };
        let gate = if effect.needs_approve { "  gated" } else { "" };
        out.push_str(&format!(
            "  {:<8} {}{}{}\n",
            effect.effect, effect.detail, taint, gate
        ));
    }

    out.push_str("\nResult  OK. This program may run.\n");
    out
}
