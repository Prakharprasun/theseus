use anyhow::Result;
use std::path::Path;

/// Derives the standard Theseus project name from the given directory path.
/// Returns `<folder_name>-theseus`.
pub fn get_project_name(current_dir: &Path) -> Result<String> {
    
    let folder_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("Could not extract folder name from current directory path"))?;
    
    // Docker expects lower case for repository names
    let safe_folder_name = folder_name.to_lowercase().replace(' ', "-");
    
    Ok(format!("{}-theseus", safe_folder_name))
}
