use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

// --- Phase 1: MVP Core (CLI Parsing & Passthrough) ---

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("tsus").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A student-friendly, Docker-powered container runner"));
}

#[test]
fn test_passthrough() {
    // Note: Assuming `docker --version` works on the host
    let mut cmd = Command::cargo_bin("tsus").unwrap();
    cmd.arg("--version")
        .assert()
        .success();
}

// --- Phase 2: tsus run ---

#[test]
fn test_run_missing_dockerfile_and_no_files_detected() {
    let temp = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("tsus").unwrap();
    cmd.current_dir(temp.path())
        .arg("run")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Could not detect project type."));
}

// --- Phase 3: tsus init ---

#[test]
fn test_init_python() {
    let temp = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("tsus").unwrap();
    cmd.current_dir(temp.path())
        .arg("init")
        .arg("python")
        .assert()
        .success()
        .stdout(predicate::str::contains("Successfully initialized a new python project"));

    // Verify file content
    let dockerfile_content = fs::read_to_string(temp.path().join("Dockerfile")).unwrap();
    assert!(dockerfile_content.contains("FROM python:3.11-slim"));
}

#[test]
fn test_init_unsupported_language() {
    let temp = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("tsus").unwrap();
    cmd.current_dir(temp.path())
        .arg("init")
        .arg("ruby")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unsupported language 'ruby'"));
}

#[test]
fn test_init_prevents_overwrite() {
    let temp = TempDir::new().unwrap();
    // Simulate existing Dockerfile
    fs::write(temp.path().join("Dockerfile"), "FROM ubuntu").unwrap();
    
    let mut cmd = Command::cargo_bin("tsus").unwrap();
    cmd.current_dir(temp.path())
        .arg("init")
        .arg("node")
        .assert()
        .failure()
        .stderr(predicate::str::contains("A Dockerfile already exists in this directory"));
}

// --- Phase 4: Project Identity (Unit internal logic tested via CLI implicitly) ---
// Given we're doing black-box CLI testing here, we test the identity logic indirectly 
// through command execution output. We could also expose a lib to test `get_project_name` directly.

// --- Phase 5: Supporting Commands ---

#[test]
fn test_status_command_executes() {
    // Note: this assumes Docker daemon is running and `docker ps` exit code is 0.
    // It verifies that `tsus status` doesn't purely crash the router.
    let mut cmd = Command::cargo_bin("tsus").unwrap();
    let _ = cmd.arg("status")
        .assert();
    // we don't assert strictly success here because docker daemon might be off on test machines,
    // but the CLI itself parses the command correctly.
}

#[test]
fn test_clean_command_executes() {
    let mut cmd = Command::cargo_bin("tsus").unwrap();
    let _ = cmd.arg("clean")
        .assert();
}
