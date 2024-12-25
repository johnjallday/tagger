use chrono::{DateTime, Local};
use serde::Serialize;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use log::{info, warn}; // Removed `error` as it's unused in this module.

use std::collections::HashSet;

pub mod programming;
pub mod music;

use programming::{extract_cargo_dependencies, generate_programming_tags};
use music::generate_music_tags;

/// Represents information about a project.
#[derive(Debug, Serialize)]
pub struct ProjectInfo {
    /// The name of the project.
    pub name: String,
    /// An alias or nickname for the project.
    pub alias: String,
    /// The type/category of the project (e.g., programming, music).
    pub project_type: String,
    /// A list of tags associated with the project.
    pub tags: Vec<String>,
    /// The creation date and time of the project.
    pub date_created: DateTime<Local>,
    /// The last modification date and time of the project.
    pub date_modified: DateTime<Local>,
    /// A list of notes related to the project.
    pub notes: Vec<String>,
}

impl ProjectInfo {
    /// Prints the project information.
    pub fn print_info(&self) {
        println!("Project Name: {}", self.name);
        println!(
            "Alias: {}",
            if self.alias.is_empty() {
                "None".to_string()
            } else {
                self.alias.clone()
            }
        );
        println!("Project Type: {}", self.project_type);
        println!("Tags: {:?}", self.tags);
        println!("Date Created: {}", self.date_created);
        println!("Date Modified: {}", self.date_modified);

        if self.notes.is_empty() {
            println!("Notes: None");
        } else {
            println!("Notes: {:?}", self.notes);
        }
    }

    /// Sets an alias for the project.
    pub fn set_alias(&mut self, alias: String) {
        self.alias = alias;
    }

    /// Adds a note to the project.
    pub fn add_note(&mut self, note: String) {
        self.notes.push(note);
    }

    /// Saves the project information to a TOML file within the specified directory.
    pub fn save_to_toml_file(&self, directory: &Path) -> io::Result<()> {
        // Convert the struct to a TOML string.
        let toml_string = toml::to_string(self).expect("Failed to serialize to TOML");

        // Define the full path for the TOML file.
        let file_path = directory.join("project_info.toml");

        // Write the TOML string to the file.
        let mut file = File::create(&file_path)?;
        file.write_all(toml_string.as_bytes())?;

        println!("✅ Saved to {}", file_path.display());
        Ok(())
    }

    /// Automatically generates ProjectInfo based on the provided directory.
    pub fn generate_project_info(directory: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Generating project information for directory: {}", directory.display());

        // Canonicalize the path to get the absolute path.
        let abs_path = fs::canonicalize(directory)?;

        // Derive the project name from the absolute path.
        let project_name = match abs_path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => "Unnamed Project".to_string(),
        };

        // Retrieve metadata from the original directory path.
        let metadata = fs::metadata(directory)?;

        // Determine the project type based on directory contents.
        let project_type = Self::generate_project_type(directory);
        info!("Project type determined as '{}'.", project_type);

        // Generate tags based on directory contents.
        let tags = Self::generate_tags(directory, &project_type)?;
        info!("Tags generated: {:?}", tags);

        // Initialize ProjectInfo with empty notes.
        Ok(ProjectInfo {
            name: project_name,
            alias: "".to_string(), // Empty alias
            project_type,
            tags,
            date_created: Self::get_creation_time(&metadata),
            date_modified: Self::get_modification_time(&metadata),
            notes: Vec::new(), // Initialize as empty
        })
    }

    /// Generates the project type based on the files in the directory.
    fn generate_project_type(directory: &Path) -> String {
        // Define indicators for different project types.
        let programming_indicators = vec![
            "Cargo.toml",
            "package.json",
            "setup.py",
            "pom.xml",
            "build.gradle",
            "Makefile",
            "Gemfile",
            "requirements.txt",
        ];

        let music_production_indicators = vec![
            "project.als",        // Ableton Live
            "project.flp",        // FL Studio
            "project.logic",      // Logic Pro
            "project.rpp",        // Reaper
            "project.studioone",  // Presonus Studio One
        ];

        // Flags to indicate project type detection.
        let mut is_programming = false;
        let mut is_music = false;

        // Collect all files in the directory (non-recursive).
        let entries = fs::read_dir(directory).unwrap_or_else(|_| {
            eprintln!("Error: Unable to read directory contents.");
            std::process::exit(1);
        });

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                // Check for specific indicator files.
                if let Some(file_name) = path.file_name() {
                    // Check for programming indicators.
                    if programming_indicators.contains(&file_name.to_string_lossy().as_ref()) {
                        is_programming = true;
                        break; // Priority can be given based on needs.
                    }

                    // Check for music production indicators.
                    if music_production_indicators.contains(&file_name.to_string_lossy().as_ref()) {
                        is_music = true;
                        break;
                    }
                }

                // Additionally, check file extensions.
                if let Some(extension) = path.extension() {
                    match extension.to_str().unwrap_or("").to_lowercase().as_str() {
                        // Programming file extensions.
                        "rs" | "py" | "js" | "java" | "cpp" | "c" | "cs" | "go" | "rb" | "swift" => {
                            is_programming = true;
                        }
                        // Music production file extensions.
                        "wav" | "mp3" | "flac" | "ogg" | "aiff" | "rpp" | "flp" | "logic" | "studioone" => {
                            is_music = true;
                        }
                        _ => {}
                    }

                    // Priority: If both types are detected, decide based on preference.
                    if is_programming && is_music {
                        break; // Stop early if both are detected.
                    }
                }
            }
        }

        if is_programming {
            info!("Detected as a programming project.");
            "programming".to_string()
        } else if is_music {
            info!("Detected as a music project.");
            "music".to_string()
        } else {
            warn!("Project type is unknown.");
            "unknown".to_string()
        }
    }

    /// Generates tags based on the files in the directory and the determined project type.
    fn generate_tags(directory: &Path, project_type: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let tags = match project_type {
            "programming" => {
                let mut prog_tags = generate_programming_tags(directory);
                // Extract dependencies and add as tags.
                //let dependencies = extract_cargo_dependencies(directory);
                //prog_tags.extend(dependencies);
                Ok::<Vec<String>, Box<dyn std::error::Error>>(prog_tags)
            },
            "music" => {
                let music_tags = generate_music_tags(directory);
                Ok::<Vec<String>, Box<dyn std::error::Error>>(music_tags)
            },
            _ => {
                let unknown_tags = Self::generate_unknown_tags(directory);
                Ok::<Vec<String>, Box<dyn std::error::Error>>(unknown_tags)
            },
        }?;

        // Remove duplicate tags by converting to a set and back.
        let unique_tags: HashSet<_> = tags.into_iter().collect();
        let mut unique_tags: Vec<String> = unique_tags.into_iter().collect();
        unique_tags.sort(); // Optional: sort tags alphabetically.

        info!("Tags after deduplication and sorting: {:?}", unique_tags);

        Ok(unique_tags)
    }

    /// Generates generic tags for unknown project types based on file extensions.
    ///
    /// # Arguments
    ///
    /// * `directory` - A reference to the project's directory path.
    ///
    /// # Returns
    ///
    /// A vector of generic tags.
    fn generate_unknown_tags(directory: &Path) -> Vec<String> {
        let mut tags = Vec::new();

        let entries = fs::read_dir(directory).unwrap_or_else(|_| {
            eprintln!("Error: Unable to read directory contents for tag generation.");
            std::process::exit(1);
        });

        let mut generic_tags = HashSet::new();

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if let Some(extension) = path.extension() {
                    if let Some(ext_str) = extension.to_str() {
                        generic_tags.insert(ext_str.to_uppercase());
                    }
                }
            }
        }

        // Add generic tags.
        tags.extend(generic_tags.into_iter());

        info!("Unknown project tags generated: {:?}", tags);

        tags
    }

    /// Fetches creation time from metadata.
    fn get_creation_time(metadata: &fs::Metadata) -> DateTime<Local> {
        match metadata.created() {
            Ok(time) => DateTime::<Local>::from(time),
            Err(_) => {
                warn!("Creation time not available. Using current time as fallback.");
                Local::now()
            }
        }
    }

    /// Fetches modification time from metadata.
    fn get_modification_time(metadata: &fs::Metadata) -> DateTime<Local> {
        match metadata.modified() {
            Ok(time) => DateTime::<Local>::from(time),
            Err(_) => {
                warn!("Modification time not available. Using current time as fallback.");
                Local::now()
            }
        }
    }
}

// Move the tests module outside the impl block.
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_generate_project_type_programming() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // Create a Cargo.toml file to indicate a Rust project.
        let cargo_toml_path = dir_path.join("Cargo.toml");
        let mut file = File::create(&cargo_toml_path).unwrap();
        writeln!(file, "[package]").unwrap();

        let project_type = ProjectInfo::generate_project_type(dir_path);
        assert_eq!(project_type, "programming");
    }

    #[test]
    fn test_generate_project_type_music() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // Create a project.rpp file to indicate a Reaper project.
        let rpp_path = dir_path.join("project.rpp");
        File::create(&rpp_path).unwrap();

        let project_type = ProjectInfo::generate_project_type(dir_path);
        assert_eq!(project_type, "music");
    }

    #[test]
    fn test_generate_project_type_unknown() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // Create some unrelated files.
        let img_path = dir_path.join("image.png");
        File::create(&img_path).unwrap();

        let doc_path = dir_path.join("document.pdf");
        File::create(&doc_path).unwrap();

        let project_type = ProjectInfo::generate_project_type(dir_path);
        assert_eq!(project_type, "unknown");
    }

    #[test]
    fn test_generate_tags_programming_with_cargo_toml() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // Create a Cargo.toml file to indicate a Rust project.
        let cargo_toml_path = dir_path.join("Cargo.toml");
        let mut file = File::create(&cargo_toml_path).unwrap();
        writeln!(file, "[package]").unwrap();

        // Create a main.rs file.
        let src_dir = dir_path.join("src");
        fs::create_dir(&src_dir).unwrap();
        let main_rs_path = src_dir.join("main.rs");
        File::create(&main_rs_path).unwrap();

        let project_type = ProjectInfo::generate_project_type(dir_path);
        assert_eq!(project_type, "programming");

        let tags = ProjectInfo::generate_tags(dir_path, &project_type).unwrap();
        assert!(tags.contains(&"rust".to_string()));
        assert!(tags.contains(&"cli".to_string()));
        assert!(tags.contains(&"software development".to_string()));
    }

    #[test]
    fn test_generate_tags_programming_without_cargo_toml() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // Create a main.rs file without Cargo.toml.
        let src_dir = dir_path.join("src");
        fs::create_dir(&src_dir).unwrap();
        let main_rs_path = src_dir.join("main.rs");
        File::create(&main_rs_path).unwrap();

        let project_type = ProjectInfo::generate_project_type(dir_path);
        assert_eq!(project_type, "programming");

        let tags = ProjectInfo::generate_tags(dir_path, &project_type).unwrap();
        assert!(tags.contains(&"rust".to_string())); // Assuming "rust" is inferred from .rs files
        assert!(tags.contains(&"cli".to_string()));
        assert!(tags.contains(&"software development".to_string()));
    }

    #[test]
    fn test_generate_tags_music() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // Create a Reaper project structure.
        let rpp_path = dir_path.join("project.rpp");
        File::create(&rpp_path).unwrap();

        let samples_dir = dir_path.join("samples");
        fs::create_dir(&samples_dir).unwrap();

        let kick_wav = samples_dir.join("kick.wav");
        File::create(&kick_wav).unwrap();

        let snare_mp3 = samples_dir.join("snare.mp3");
        File::create(&snare_mp3).unwrap();

        let tags = ProjectInfo::generate_tags(dir_path, "music").unwrap();
        assert!(tags.contains(&"WAV".to_string()));
        assert!(tags.contains(&"MP3".to_string()));
        assert!(tags.contains(&"RPP".to_string()));
        assert!(tags.contains(&"Reaper".to_string()));
        assert!(tags.contains(&"audio".to_string()));
        assert!(tags.contains(&"production".to_string()));
    }

    #[test]
    fn test_generate_tags_unknown() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // Create some unrelated files.
        let img_path = dir_path.join("image.png");
        File::create(&img_path).unwrap();

        let doc_path = dir_path.join("document.pdf");
        File::create(&doc_path).unwrap();

        let tags = ProjectInfo::generate_tags(dir_path, "unknown").unwrap();
        assert!(tags.contains(&"IMAGE".to_string()));
        assert!(tags.contains(&"PDF".to_string()));
    }
}
