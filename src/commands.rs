use anyhow::{bail, Result};
use std::env;
use std::process::{Command, Stdio};

use crate::project;

pub fn stop() -> Result<()> {
    let current_dir = env::current_dir()?;
    let project_name = project::get_project_name(&current_dir)?;
    println!("Stopping container {}...", project_name);

    let stop_status = Command::new("docker")
        .args(["stop", &project_name])
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .status()?;

    if stop_status.success() {
        println!("✅ Stopped container.");
    } else {
        println!("Container might not have been running.");
    }

    Ok(())
}

pub fn status() -> Result<()> {
    let current_dir = env::current_dir()?;
    let project_name = project::get_project_name(&current_dir)?;
    println!("--- Theseus Project Status ---");
    println!("Project Identity: {}", project_name);

    // Check if container is running
    let ps_output = Command::new("docker")
        .args([
            "ps",
            "--filter",
            &format!("name={}", project_name),
            "--format",
            "{{.Status}} - Ports: {{.Ports}}",
        ])
        .output()?;

    let ps_str = String::from_utf8_lossy(&ps_output.stdout);
    if ps_str.trim().is_empty() {
        println!("Container: Not running");
    } else {
        println!("Container: Running ({})", ps_str.trim());
    }

    // Check if image exists
    let img_output = Command::new("docker")
        .args([
            "images",
            "--filter",
            &format!("reference={}", project_name),
            "--format",
            "{{.Size}}",
        ])
        .output()?;

    let img_str = String::from_utf8_lossy(&img_output.stdout);
    if img_str.trim().is_empty() {
        println!("Image: Not built yet");
    } else {
        println!("Image Size: {}", img_str.trim());
    }

    Ok(())
}

pub fn clean(all: bool) -> Result<()> {
    let current_dir = env::current_dir()?;
    let project_name = project::get_project_name(&current_dir)?;
    
    // Stop if running
    let _ = Command::new("docker")
        .args(["stop", &project_name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    // RMI
    println!("Removing project image...");
    let rmi_status = Command::new("docker")
        .args(["rmi", "-f", &project_name])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if rmi_status.success() {
        println!("✅ Cleaned project.");
    }

    if all {
        println!("\nPruning all unused Docker data...");
        let prune_status = Command::new("docker")
            .args(["system", "prune", "-f"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;
        
        if !prune_status.success() {
            bail!("System prune failed.");
        }
    }

    Ok(())
}

pub fn logs() -> Result<()> {
    let current_dir = env::current_dir()?;
    let project_name = project::get_project_name(&current_dir)?;
    
    let status = Command::new("docker")
        .args(["logs", "-f", &project_name])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if let Some(code) = status.code() {
        if code != 0 {
            std::process::exit(code);
        }
    }
    
    Ok(())
}

pub fn exec() -> Result<()> {
    let current_dir = env::current_dir()?;
    let project_name = project::get_project_name(&current_dir)?;
    println!("Opening shell in container {}...", project_name);

    let status = Command::new("docker")
        .args(["exec", "-it", &project_name, "/bin/sh"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        bail!("Failed to execute shell. Is the container running?");
    }

    Ok(())
}

pub fn doctor() -> Result<()> {
    println!("🩺 Theseus Diagnostics\n");

    let mut all_good = true;

    // 1. Check if Docker is installed
    print!("Checking Docker installation... ");
    let docker_installed = Command::new("docker")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if let Ok(status) = docker_installed {
        if status.success() {
            println!("✅ OK");
        } else {
            println!("❌ Failed (status code: {})", status.code().unwrap_or(-1));
            all_good = false;
        }
    } else {
        println!("❌ Not found");
        all_good = false;
    }

    // 2. Check if Docker daemon is running
    print!("Checking Docker daemon... ");
    let docker_running = Command::new("docker")
        .arg("info")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if let Ok(status) = docker_running {
        if status.success() {
            println!("✅ OK");
        } else {
            println!("❌ Not running");
            all_good = false;
        }
    } else {
        println!("❌ Error checking daemon");
        all_good = false;
    }

    // 3. Check Theseus binary location (optional sanity check)
    print!("Checking Theseus installation... ");
    if let Ok(exe_path) = env::current_exe() {
        println!("✅ Installed at {}", exe_path.display());
    } else {
        println!("❌ Could not determine executable path");
    }

    println!("\n--- Result ---");
    if all_good {
        println!("🎉 Your system is ready for Theseus!");
    } else {
        println!("⚠️ There are issues with your Docker setup.");
        println!("Please ensure Docker is installed and running, then try again.");
    }

    Ok(())
}
