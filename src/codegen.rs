use std::fs;
use std::path::Path;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::ast::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompiledUnit {
    pub language: String,
    pub version: String,
    pub source_path: String,
    pub program: Program,
}

pub fn compile_trell_package(program: &Program, source_path: &Path, output_path: &Path) -> Result<()> {
    let unit = CompiledUnit {
        language: "Trell".to_string(),
        version: "0.2.0".to_string(),
        source_path: source_path.display().to_string(),
        program: program.clone(),
    };

    let serialized = serde_json::to_string_pretty(&unit)?;
    fs::write(output_path, serialized)?;
    Ok(())
}
