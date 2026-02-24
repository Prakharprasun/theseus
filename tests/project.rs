use std::fs;
use tempfile::TempDir;
use theseus::project;

#[test]
fn test_get_project_name_normal() {
    let temp = TempDir::new().unwrap();
    let unique_dir = temp.path().join("my-rust-app");
    fs::create_dir(&unique_dir).unwrap();

    let name = project::get_project_name(&unique_dir).unwrap();
    assert_eq!(name, "my-rust-app-theseus");
}

#[test]
fn test_get_project_name_with_spaces_and_caps() {
    let temp = TempDir::new().unwrap();
    let unique_dir = temp.path().join("My Project File");
    fs::create_dir(&unique_dir).unwrap();

    let name = project::get_project_name(&unique_dir).unwrap();
    // Verify it lowercases and converts spaces to dashes inside the string
    assert_eq!(name, "my-project-file-theseus");
}
