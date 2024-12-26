use project_info::music::generate_music_tags;
use tempfile::tempdir;
use std::fs::{self, File};

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

    let tags = generate_music_tags(dir_path);
    assert!(tags.contains(&"WAV".to_string()));
    assert!(tags.contains(&"MP3".to_string()));
    assert!(tags.contains(&"RPP".to_string()));
    assert!(tags.contains(&"Reaper".to_string()));
    assert!(tags.contains(&"audio".to_string()));
    assert!(tags.contains(&"production".to_string()));
}
