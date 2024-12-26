use project_info::ProjectInfo;
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
