use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::interpreter::{BeliefValue, ModelOracle, RuntimeValue};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockScenarioFile {
    pub responses: HashMap<String, MockScenarioEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockScenarioEntry {
    pub value: String,
    pub confidence: f64,
    pub justification: String,
}

pub struct ConfigurableOracle {
    entries: HashMap<String, MockScenarioEntry>,
}

impl ConfigurableOracle {
    pub fn new() -> Self {
        let mut entries = HashMap::new();
        // Default entries
        entries.insert("assess_medical".to_string(), MockScenarioEntry {
            value: "BacterialInfection".to_string(),
            confidence: 0.94,
            justification: "Biomarker analysis indicates localized bacterial proliferation requiring targeted antibiotic protocol".to_string(),
        });
        entries.insert("evaluate_risk".to_string(), MockScenarioEntry {
            value: "ApproveTransfer".to_string(),
            confidence: 0.89,
            justification: "Account history and transaction velocity correlate with verified low-risk corporate settlement profile".to_string(),
        });
        entries.insert("audit_code".to_string(), MockScenarioEntry {
            value: "ApprovedPatch".to_string(),
            confidence: 0.97,
            justification: "Speculative static analysis confirms no memory unsafety, unbound recursion, or unauthorized syscall escape".to_string(),
        });
        entries.insert("lookoutai".to_string(), MockScenarioEntry {
            value: "VeerStarboard".to_string(),
            confidence: 0.94,
            justification: "COLREGs Rule 14: Head-on situation detected, execute standard starboard alteration to pass port-to-port".to_string(),
        });
        entries.insert("fraudoracle".to_string(), MockScenarioEntry {
            value: "ClearWire".to_string(),
            confidence: 0.93,
            justification: "Consensus quorum verified: wire beneficiary matches authenticated recipient directory with standard velocity".to_string(),
        });
        entries.insert("vessel".to_string(), MockScenarioEntry {
            value: "VeerStarboard".to_string(),
            confidence: 0.94,
            justification: "COLREGs Rule 14: Head-on situation detected, execute standard starboard alteration to pass port-to-port".to_string(),
        });
        entries.insert("wire".to_string(), MockScenarioEntry {
            value: "ClearWire".to_string(),
            confidence: 0.93,
            justification: "Consensus quorum verified: wire beneficiary matches authenticated recipient directory with standard velocity".to_string(),
        });

        Self { entries }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let scenario: MockScenarioFile = serde_json::from_str(&content)?;
        Ok(Self { entries: scenario.responses })
    }

    pub fn register(&mut self, key: &str, value: &str, confidence: f64, justification: &str) {
        self.entries.insert(key.to_string(), MockScenarioEntry {
            value: value.to_string(),
            confidence,
            justification: justification.to_string(),
        });
    }
}

impl ModelOracle for ConfigurableOracle {
    fn query(&mut self, contract: &crate::ast::ModelContract, method: &str, prompt: &str) -> Result<BeliefValue> {
        // Match on method or prompt
        let entry = if let Some(e) = self.entries.get(method) {
            e.clone()
        } else {
            let mut found: Option<MockScenarioEntry> = None;
            for (k, v) in &self.entries {
                if prompt.to_lowercase().contains(&k.to_lowercase()) {
                    found = Some(v.clone());
                    break;
                }
            }
            match found {
                Some(entry) => entry,
                None => MockScenarioEntry {
                    value: "SemanticConsensusVerified".to_string(),
                    confidence: 0.91,
                    justification: format!("Oracle deliberated under contract '{}' using temperature {:?}", contract.name, contract.temperature),
                },
            }
        };

        if let Some(min_conf) = contract.min_confidence {
            if entry.confidence < min_conf {
                return Err(anyhow!(
                    "Cognitive Invariant Violation: Model confidence {:.2} violates contract '{}' minimum of {:.2}",
                    entry.confidence,
                    contract.name,
                    min_conf
                ));
            }
        }

        Ok(BeliefValue {
            value: Box::new(RuntimeValue::String(entry.value)),
            confidence: entry.confidence,
            justification: entry.justification,
            model_origin: contract.name.clone(),
        })
    }
}
