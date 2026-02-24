use anyhow::{bail, format_err, Result};
use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::project;
use crate::templates;

#[derive(Debug, PartialEq)]
pub enum Language {
    Node,
    Python,
    Rust,
    Go,
}

pub fn execute() -> Result<()> {
    let current_dir = env::current_dir()?;
    let current_dir_str = current_dir.to_str().unwrap();

    let project_name = project::get_project_name(&current_dir)?;
    

    let lang = match Path::new("Dockerfile").exists() {
        true => {
            println!("Found existing Dockerfile. Using it.");
            None
        }
        false => {
            println!("No Dockerfile found. Detecting project type...");
            let lang = detect_language(&current_dir)?;
            
            let template = match lang {
                Language::Node => {
                    println!("Detected Node.js project. Using temporary development container.");
                    templates::NODE_TEMPLATE
                }
                Language::Python => {
                    println!("Detected Python project. Using temporary development container.");
                    templates::PYTHON_TEMPLATE
                }
                Language::Rust => {
                    println!("Detected Rust project. Using temporary development container.");
                    templates::RUST_TEMPLATE
                }
                Language::Go => {
                    println!("Detected Go project. Using temporary development container.");
                    templates::GO_TEMPLATE
                }
            };

            fs::write(".theseus.Dockerfile", template)
                .map_err(|e| format_err!("Failed to write temporary Dockerfile: {}", e))?;
                
            Some(lang)
        }
    };
    
    let dockerfile_path = if lang.is_some() { ".theseus.Dockerfile" } else { "Dockerfile" };

    // 2. Build the image
    println!("Building image {}...", project_name);
    let build_status = Command::new("docker")
        .args(["build", "-f", dockerfile_path, "-t", &project_name, "."])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !build_status.success() {
        bail!("Docker build failed.");
    }

    println!("Starting container {}...", project_name);
    let mut run_cmd = Command::new("docker");
    run_cmd.args([
        "run",
        "-it",
        "--rm",
        "-v",
        &format!("{}:/app", current_dir_str),
        "-w",
        "/app",
        "--name",
        &project_name,
        &project_name,
    ]);

    let run_status = run_cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .status()?;

    if !run_status.success() {
        if let Some(code) = run_status.code() {
            std::process::exit(code);
        } else {
            std::process::exit(1);
        }
    }

    Ok(())
}

fn detect_language(dir: &Path) -> Result<Language> {
    if dir.join("package.json").exists() {
        return Ok(Language::Node);
    }
    if dir.join("requirements.txt").exists() || dir.join("pyproject.toml").exists() {
        return Ok(Language::Python);
    }
    if dir.join("Cargo.toml").exists() {
        return Ok(Language::Rust);
    }
    if dir.join("go.mod").exists() {
        return Ok(Language::Go);
    }

    // If we reach here, we couldn't detect the project type
    let error_msg = "\nCould not detect project type.

Theseus looked for:
- package.json (Node)
- requirements.txt / pyproject.toml (Python)
- Cargo.toml (Rust)
- go.mod (Go)

To continue, you can:
1) Create one of the above files
2) Run: tsus init <language>
3) Add a Dockerfile manually

Example:
  tsus init python
";
    bail!("{}", error_msg);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;

    #[test]
    fn test_detect_node() {
        let temp = TempDir::new().unwrap();
        File::create(temp.path().join("package.json")).unwrap();
        assert_eq!(detect_language(temp.path()).unwrap(), Language::Node);
    }

    #[test]
    fn test_detect_python_requirements() {
        let temp = TempDir::new().unwrap();
        File::create(temp.path().join("requirements.txt")).unwrap();
        assert_eq!(detect_language(temp.path()).unwrap(), Language::Python);
    }

    #[test]
    fn test_detect_python_pyproject() {
        let temp = TempDir::new().unwrap();
        File::create(temp.path().join("pyproject.toml")).unwrap();
        assert_eq!(detect_language(temp.path()).unwrap(), Language::Python);
    }

    #[test]
    fn test_detect_rust() {
        let temp = TempDir::new().unwrap();
        File::create(temp.path().join("Cargo.toml")).unwrap();
        assert_eq!(detect_language(temp.path()).unwrap(), Language::Rust);
    }

    #[test]
    fn test_detect_go() {
        let temp = TempDir::new().unwrap();
        File::create(temp.path().join("go.mod")).unwrap();
        assert_eq!(detect_language(temp.path()).unwrap(), Language::Go);
    }

    #[test]
    fn test_detect_failure() {
        let temp = TempDir::new().unwrap();
        File::create(temp.path().join("main.c")).unwrap();
        let result = detect_language(temp.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Could not detect project type"));
    }
}
