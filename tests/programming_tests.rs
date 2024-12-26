use project_info::programming::generate_programming_tags;
use tempfile::tempdir;
use std::fs::{self, File};

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

    let tags = generate_programming_tags(dir_path);
    assert!(tags.contains(&"rust".to_string()));
    assert!(tags.contains(&"cli".to_string()));
    assert!(tags.contains(&"software development".to_string()));
}
