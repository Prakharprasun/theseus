use anyhow::{bail, format_err, Result};
use std::fs;
use std::path::Path;

use crate::templates;

pub fn execute(language: &str) -> Result<()> {
    if Path::new("Dockerfile").exists() {
        bail!("A Dockerfile already exists in this directory.\nTheseus will not overwrite it. If you want to re-initialize from a template, please remove the existing Dockerfile first.");
    }

    let template = match language.to_lowercase().as_str() {
        "node" => templates::NODE_TEMPLATE,
        "python" => templates::PYTHON_TEMPLATE,
        "rust" => templates::RUST_TEMPLATE,
        "go" => templates::GO_TEMPLATE,
        other => bail!(
            "Unsupported language '{}'.\nSupported languages are: node, python, rust, go.",
            other
        ),
    };

    fs::write("Dockerfile", template)
        .map_err(|e| format_err!("Failed to write Dockerfile: {}", e))?;

    println!("✅ Successfully initialized a new {} project with a development Dockerfile.", language);
    println!("Next steps:\n  1. Add your code\n  2. Run `tsus run` to see it in action!");

    Ok(())
}
