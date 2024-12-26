use chrono::{DateTime, Local};
use serde::Serialize;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use log::{info, warn}; // Removed `error` as it's unused in this module.

use std::collections::HashSet;

pub mod programming;
pub mod music;

use programming::{extract_git_push_url, generate_programming_tags};
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
    /// The Git push URL of the project (if applicable).
    pub git_url: Option<String>,
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

        if let Some(git_url) = &self.git_url {
            println!("Git URL: {}", git_url);
        } else {
            println!("Git URL: None");
        }

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

        // Extract Git push URL (if the project is a Git repository).
        let git_url = extract_git_push_url(directory);

        // Initialize ProjectInfo with empty notes.
        Ok(ProjectInfo {
            name: project_name,
            alias: "".to_string(), // Empty alias
            project_type,
            tags,
            date_created: Self::get_creation_time(&metadata),
            date_modified: Self::get_modification_time(&metadata),
            notes: Vec::new(), // Initialize as empty
            git_url, // Add the Git push URL here
        })
    }

    /// Generates the project type based on the files in the directory.
    fn generate_project_type(directory: &Path) -> String {
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
            "project.als", "project.flp", "project.logic", "project.rpp", "project.studioone",
        ];

        let mut is_programming = false;
        let mut is_music = false;

        let entries = fs::read_dir(directory).unwrap_or_else(|_| {
            eprintln!("Error: Unable to read directory contents.");
            std::process::exit(1);
        });

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if let Some(file_name) = path.file_name() {
                    if programming_indicators.contains(&file_name.to_string_lossy().as_ref()) {
                        is_programming = true;
                        break;
                    }

                    if music_production_indicators.contains(&file_name.to_string_lossy().as_ref()) {
                        is_music = true;
                        break;
                    }
                }

                if let Some(extension) = path.extension() {
                    match extension.to_str().unwrap_or("").to_lowercase().as_str() {
                        "rs" | "py" | "js" | "java" | "cpp" | "c" | "cs" | "go" | "rb" | "swift" => {
                            is_programming = true;
                        }
                        "wav" | "mp3" | "flac" | "ogg" | "aiff" | "rpp" | "flp" | "logic" | "studioone" => {
                            is_music = true;
                        }
                        _ => {}
                    }

                    if is_programming && is_music {
                        break;
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

        let unique_tags: HashSet<_> = tags.into_iter().collect();
        let mut unique_tags: Vec<String> = unique_tags.into_iter().collect();
        unique_tags.sort();

        info!("Tags after deduplication and sorting: {:?}", unique_tags);

        Ok(unique_tags)
    }

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

        tags.extend(generic_tags.into_iter());
        info!("Unknown project tags generated: {:?}", tags);

        tags
    }

    fn get_creation_time(metadata: &fs::Metadata) -> DateTime<Local> {
        match metadata.created() {
            Ok(time) => DateTime::<Local>::from(time),
            Err(_) => {
                warn!("Creation time not available. Using current time as fallback.");
                Local::now()
            }
        }
    }

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
