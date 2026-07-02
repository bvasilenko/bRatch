mod common;
use bsuite_core::ExitCode;
use common::bratch_command;
use tempfile::TempDir;

#[cfg(unix)]
#[test]
fn directive_still_emits_when_transcript_directory_is_not_writable() {
    use std::os::unix::fs::PermissionsExt as _;

    let dir = TempDir::new().unwrap();
    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o444))
        .expect("chmod 444 on temp dir");

    let output = bratch_command()
        .env("BSUITE_TRANSCRIPT_DIR", dir.path())
        .args(["compare", "--signature", "null-check-removed"])
        .assert()
        .code(ExitCode::Finding.as_i32())
        .get_output()
        .clone();

    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o755))
        .expect("restore permissions for cleanup");

    assert!(
        !output.stdout.is_empty(),
        "directive must still reach stdout when transcript write fails"
    );
    assert!(
        !output.stderr.is_empty(),
        "transcript write failure must surface on stderr"
    );
}

#[cfg(unix)]
#[test]
fn existing_transcript_unchanged_after_subsequent_write_failure() {
    use std::os::unix::fs::PermissionsExt as _;

    let dir = TempDir::new().unwrap();

    // Pre-seed a transcript file from a past date so bCore's date-named write targets a
    // different path and cannot overwrite this file.
    let bratch_dir = dir.path().join("bratch");
    std::fs::create_dir(&bratch_dir).expect("create bratch subdir");
    let seed_path = bratch_dir.join("2020-01-01.jsonl");
    let seed_content = b"{\"sentinel\":true}\n";
    std::fs::write(&seed_path, seed_content).expect("write seed transcript");

    std::fs::set_permissions(&bratch_dir, std::fs::Permissions::from_mode(0o555))
        .expect("chmod 555 on bratch subdir");

    bratch_command()
        .env("BSUITE_TRANSCRIPT_DIR", dir.path())
        .args(["compare", "--signature", "null-check-removed"])
        .assert()
        .code(ExitCode::Finding.as_i32());

    std::fs::set_permissions(&bratch_dir, std::fs::Permissions::from_mode(0o755))
        .expect("restore permissions for cleanup");

    let after = std::fs::read(&seed_path).expect("read seed path after run");
    assert_eq!(
        seed_content,
        after.as_slice(),
        "pre-existing transcript must be byte-identical after a failed write"
    );

    let jsonl_count = std::fs::read_dir(&bratch_dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|x| x.to_str())
                .is_some_and(|ext| ext == "jsonl")
        })
        .count();
    assert_eq!(
        1, jsonl_count,
        "only the pre-seeded JSONL must exist; no new file may have been created"
    );
}
