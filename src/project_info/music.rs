use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use log::{info, warn};

/// Generates tags specific to music projects based on the directory contents.
///
/// # Arguments
///
/// * `directory` - A reference to the project's directory path.
///
/// # Returns
///
/// A vector of tags relevant to music projects.
pub fn generate_music_tags(directory: &Path) -> Vec<String> {
    let mut tags = Vec::new();

    // Define common audio formats.
    let audio_extensions = vec!["wav", "mp3", "flac", "ogg", "aiff"];

    // Define common DAWs (Digital Audio Workstations).
    let daws = vec![
        "ableton live",
        "fl studio",
        "logic pro",
        "reaper",
        "presonus studio one",
    ];

    let entries = fs::read_dir(directory).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read directory contents for tag generation.");
        std::process::exit(1);
    });

    let mut audio_format_set = HashSet::new();
    let mut daw_set = HashSet::new();

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();

            if let Some(extension) = path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    for &audio_ext in &audio_extensions {
                        if ext_str.eq_ignore_ascii_case(audio_ext) {
                            audio_format_set.insert(audio_ext.to_uppercase()); // e.g., "WAV"
                        }
                    }
                }
            }

            if let Some(file_name) = path.file_name() {
                let file_name_str = file_name.to_string_lossy().to_lowercase();
                for daw in &daws {
                    if file_name_str.contains(&daw.to_lowercase()) {
                        daw_set.insert(daw.to_string()); // e.g., "reaper"
                    }
                }
            }
        }
    }

    // Add detected audio formats and DAWs as tags.
    tags.extend(audio_format_set.into_iter());
    tags.extend(daw_set.into_iter());

    // Add general music production tags.
    tags.push("audio".to_string());
    tags.push("production".to_string());

    info!("Music tags generated: {:?}", tags);

    tags
}
