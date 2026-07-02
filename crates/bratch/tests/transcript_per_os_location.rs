mod common;
use bsuite_core::ExitCode;
use common::bratch_command;
use serde_json::Value;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn collect_jsonl_files(base: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(base)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("jsonl"))
        .collect()
}

fn assert_transcript_is_valid_jsonl(path: &Path) {
    let raw = std::fs::read_to_string(path).expect("transcript is readable");
    let record: Value =
        serde_json::from_str(raw.trim()).expect("transcript round-trips through serde_json");
    assert_eq!(
        record["binary_name"].as_str(),
        Some("bratch"),
        "binary_name field"
    );
    assert_eq!(
        record["schema_version"].as_u64(),
        Some(1),
        "schema_version field"
    );
    assert!(
        record["invocation_id"].as_str().is_some(),
        "invocation_id field"
    );
    assert!(record["timestamp"].as_str().is_some(), "timestamp field");
}

#[cfg(target_os = "linux")]
#[test]
fn transcript_lands_under_xdg_state_home_on_linux() {
    let dir = TempDir::new().unwrap();

    bratch_command()
        .env("XDG_STATE_HOME", dir.path())
        .env_remove("BSUITE_TRANSCRIPT_DIR")
        .args(["compare", "--signature", "null-check-removed"])
        .assert()
        .code(ExitCode::Finding.as_i32());

    // bCore resolves to /bsuite/transcripts/bratch/
    let bratch_dir = dir.path().join("bsuite").join("transcripts").join("bratch");
    let files = collect_jsonl_files(&bratch_dir);
    assert_eq!(
        1,
        files.len(),
        "exactly one transcript JSONL must exist under          /bsuite/transcripts/bratch/; found: {:?}",
        files
    );
    assert_transcript_is_valid_jsonl(&files[0]);
}

#[cfg(target_os = "macos")]
#[test]
fn transcript_lands_under_application_support_on_macos() {
    let dir = TempDir::new().unwrap();
    std::fs::create_dir_all(dir.path().join("Library").join("Application Support")).unwrap();

    bratch_command()
        .env("HOME", dir.path())
        .env_remove("BSUITE_TRANSCRIPT_DIR")
        .args(["compare", "--signature", "null-check-removed"])
        .assert()
        .code(ExitCode::Finding.as_i32());

    let bratch_dir = dir
        .path()
        .join("Library")
        .join("Application Support")
        .join("bsuite")
        .join("transcripts")
        .join("bratch");
    let files = collect_jsonl_files(&bratch_dir);
    assert_eq!(
        1,
        files.len(),
        "exactly one transcript JSONL must exist under          ~/Library/Application Support/bsuite/transcripts/bratch/; found: {:?}",
        files
    );
    assert_transcript_is_valid_jsonl(&files[0]);
}

#[cfg(target_os = "windows")]
#[test]
fn transcript_lands_under_localappdata_on_windows() {
    let dir = TempDir::new().unwrap();

    bratch_command()
        .env("LOCALAPPDATA", dir.path())
        .env_remove("BSUITE_TRANSCRIPT_DIR")
        .args(["compare", "--signature", "null-check-removed"])
        .assert()
        .code(ExitCode::Finding.as_i32());

    let bratch_dir = dir.path().join("bsuite").join("transcripts").join("bratch");
    let files = collect_jsonl_files(&bratch_dir);
    assert_eq!(
        1,
        files.len(),
        "exactly one transcript JSONL must exist under          %LOCALAPPDATA%/bsuite/transcripts/bratch/; found: {:?}",
        files
    );
    assert_transcript_is_valid_jsonl(&files[0]);
}
