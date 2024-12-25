mod project_info;

use project_info::ProjectInfo;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use log::{info, warn, error};
use env_logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger.
    env_logger::init();

    // Collect command-line arguments.
    let args: Vec<String> = env::args().collect();

    // Expect exactly one argument: the directory path.
    if args.len() != 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        std::process::exit(1);
    }

    let dir_path = Path::new(&args[1]);

    // Validate that the path exists and is a directory.
    if !dir_path.exists() {
        error!("The path '{}' does not exist.", dir_path.display());
        std::process::exit(1);
    }

    if !dir_path.is_dir() {
        error!("The path '{}' is not a directory.", dir_path.display());
        std::process::exit(1);
    }

    // Automatically generate ProjectInfo using the generate_project_info function.
    let mut project = ProjectInfo::generate_project_info(dir_path)?; // Make project mutable.

    info!("Project information generated successfully.");

    // Print project information.
    project.print_info();

    // Prompt for alias.
    println!("Enter an alias for the project (or press Enter to skip):");
    io::stdout().flush()?;
    let mut alias_input = String::new();
    io::stdin().read_line(&mut alias_input)?;
    let alias = alias_input.trim().to_string();
    if !alias.is_empty() {
        project.set_alias(alias);
        info!("Alias set for the project.");
    }

    // Prompt for notes.
    println!("Enter a note for the project (or press Enter to skip):");
    io::stdout().flush()?;
    let mut note_input = String::new();
    io::stdin().read_line(&mut note_input)?;
    let note = note_input.trim().to_string();
    if !note.is_empty() {
        project.add_note(note);
        info!("Note added to the project.");
    }

    // Prompt the user to decide whether to save the project information.
    loop {
        println!("\nDo you want to save this project information to 'project_info.toml'? (y/n):");

        // Flush stdout to ensure the prompt is displayed.
        io::stdout().flush()?;

        // Read user input.
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim().to_lowercase();
        match input.as_str() {
            "y" | "yes" => {
                // Attempt to save the project info.
                if let Err(e) = project.save_to_toml_file(dir_path) {
                    error!("Error saving project_info.toml: {}", e);
                    std::process::exit(1);
                }
                info!("Project information saved successfully.");
                break;
            }
            "n" | "no" => {
                println!("❌ Project information not saved.");
                info!("User chose not to save the project information.");
                break;
            }
            _ => {
                println!("Invalid input. Please enter 'y' or 'n'.");
                warn!("User provided invalid input: {}", input);
                continue;
            }
        }
    }

    Ok(())
}
