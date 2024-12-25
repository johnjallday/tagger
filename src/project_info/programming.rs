use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

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

/// Extracts dependencies from Cargo.toml and adds them as tags.
///
/// # Arguments
///
/// * `directory` - A reference to the project's directory path.
///
/// # Returns
///
/// A vector of dependency tags.
pub fn extract_cargo_dependencies(directory: &Path) -> Vec<String> {
    let mut dependencies = Vec::new();

    let cargo_toml_path = directory.join("Cargo.toml");
    if cargo_toml_path.exists() {
        if let Ok(content) = fs::read_to_string(&cargo_toml_path) {
            if let Ok(parsed) = content.parse::<toml::Value>() {
                if let Some(deps) = parsed.get("dependencies") {
                    if let Some(deps_table) = deps.as_table() {
                        for (dep, _) in deps_table.iter() {
                            dependencies.push(dep.to_lowercase());
                        }
                    }
                }
            }
        }
    }

    dependencies
}
