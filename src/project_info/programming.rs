use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;
use log::{info, warn};

/// Generates tags specific to programming projects based on the directory contents.
///
/// # Arguments
///
/// * `directory` - A reference to the project's directory path.
///
/// # Returns
///
/// A vector of tags relevant to programming projects.
pub fn generate_programming_tags(directory: &Path) -> Vec<String> {
    let mut tags = Vec::new();

    // Define programming languages and their corresponding file extensions.
    let programming_extensions = vec![
        ("rust", "rs"),
        ("python", "py"),
        ("javascript", "js"),
        ("java", "java"),
        ("cpp", "cpp"),
        ("c", "c"),
        ("c#", "cs"),
        ("go", "go"),
        ("ruby", "rb"),
        ("swift", "swift"),
    ];

    // Collect tags based on detected file extensions.
    let entries = fs::read_dir(directory).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read directory contents for tag generation.");
        std::process::exit(1);
    });

    let mut language_set = HashSet::new();

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();

            if let Some(extension) = path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    for (language, ext) in &programming_extensions {
                        if ext_str.eq_ignore_ascii_case(ext) {
                            language_set.insert(language.to_string());
                        }
                    }
                }
            }
        }
    }

    // Add detected languages as tags.
    tags.extend(language_set.into_iter());

    // Add general programming tags.
    tags.push("cli".to_string());
    tags.push("software development".to_string());

    // **Condition**: If `Cargo.toml` exists, add the "rust" tag.
    let cargo_toml_path = directory.join("Cargo.toml");
    if cargo_toml_path.exists() {
        tags.push("rust".to_string());
        info!("Detected Cargo.toml. Added 'rust' tag.");
    } else {
        info!("Cargo.toml not found. 'rust' tag not added.");
    }

    info!("Programming tags generated: {:?}", tags);

    tags
}


pub fn extract_git_push_url(directory: &Path) -> Option<String> {
    if !directory.join(".git").exists() {
        warn!("Directory is not a Git repository: {}", directory.display());
        return None;
    }

    // Get the list of remotes to find a suitable one
    let remotes_output = Command::new("git")
        .args(&["remote"])
        .current_dir(directory)
        .output();

    if let Ok(output) = remotes_output {
        if output.status.success() {
            // Store the output in a String to ensure it has a longer lifetime
            let remotes_str = String::from_utf8_lossy(&output.stdout).to_string();
            let remotes = remotes_str.lines().collect::<Vec<&str>>();

            if !remotes.is_empty() {
                let remote_name = remotes[0]; // Use the first remote (e.g., `origin`)

                // Get the push URL for the remote
                let push_url_output = Command::new("git")
                    .args(&["remote", "get-url", "--push", remote_name])
                    .current_dir(directory)
                    .output();

                if let Ok(output) = push_url_output {
                    if output.status.success() {
                        let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        info!("Git push URL detected: {}", url);
                        return Some(url);
                    } else {
                        warn!(
                            "Failed to retrieve push URL for remote '{}': {}",
                            remote_name,
                            String::from_utf8_lossy(&output.stderr)
                        );
                    }
                }
            } else {
                warn!("No remotes found in Git repository: {}", directory.display());
            }
        } else {
            warn!(
                "Failed to list remotes: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    None
}
