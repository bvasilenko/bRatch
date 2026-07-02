mod common;
use bsuite_core::ExitCode;
use common::bratch_command;
use serde_json::Value;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

struct TranscriptCase {
    name: &'static str,
    args: &'static [&'static str],
    expected_exit_code: ExitCode,
    directive_emitted: bool,
}

const TRANSCRIPT_CASES: &[TranscriptCase] = &[
    TranscriptCase {
        name: "compare-finding",
        args: &["compare", "--signature", "null-check-removed"],
        expected_exit_code: ExitCode::Finding,
        directive_emitted: true,
    },
    TranscriptCase {
        name: "compare-malformed",
        args: &["compare", "--signature", "unknown-signature"],
        expected_exit_code: ExitCode::Usage,
        directive_emitted: false,
    },
    TranscriptCase {
        name: "signatures",
        args: &["signatures"],
        expected_exit_code: ExitCode::Success,
        directive_emitted: false,
    },
    TranscriptCase {
        name: "init",
        args: &["init"],
        expected_exit_code: ExitCode::Success,
        directive_emitted: false,
    },
    TranscriptCase {
        name: "tail",
        args: &["tail"],
        expected_exit_code: ExitCode::Success,
        directive_emitted: false,
    },
    TranscriptCase {
        name: "explain",
        args: &["explain"],
        expected_exit_code: ExitCode::Success,
        directive_emitted: false,
    },
];

fn run_in_transcript_dir(args: &[&str], dir: &TempDir) -> assert_cmd::assert::Assert {
    let mut cmd = bratch_command();
    cmd.env("BSUITE_TRANSCRIPT_DIR", dir.path());
    for arg in args {
        cmd.arg(arg);
    }
    cmd.assert()
}

fn collect_transcript_files(dir: &TempDir) -> Vec<PathBuf> {
    let bratch_dir = dir.path().join("bratch");
    if !bratch_dir.exists() {
        return vec![];
    }
    std::fs::read_dir(&bratch_dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("jsonl"))
        .collect()
}

fn read_transcript_record(path: &Path) -> Value {
    let content = std::fs::read_to_string(path).expect("transcript file is readable");
    serde_json::from_str(content.trim()).expect("transcript file is valid JSON")
}

#[test]
fn every_subcommand_appends_exactly_one_transcript_record() {
    for case in TRANSCRIPT_CASES {
        let dir = TempDir::new().unwrap();
        run_in_transcript_dir(case.args, &dir).code(case.expected_exit_code.as_i32());

        let files = collect_transcript_files(&dir);
        assert_eq!(
            1,
            files.len(),
            "{}: expected 1 transcript file, got {}",
            case.name,
            files.len()
        );

        let record = read_transcript_record(&files[0]);
        assert_eq!(
            record["binary_name"].as_str(),
            Some("bratch"),
            "{}: binary_name",
            case.name
        );
        assert_eq!(
            record["schema_version"].as_u64(),
            Some(1),
            "{}: schema_version",
            case.name
        );
        assert_eq!(
            record["directive_emitted"].as_bool(),
            Some(case.directive_emitted),
            "{}: directive_emitted",
            case.name
        );
        assert_eq!(
            record["exit_code"].as_u64(),
            Some(case.expected_exit_code.as_i32() as u64),
            "{}: exit_code",
            case.name
        );
        assert!(
            record["invocation_id"].as_str().is_some(),
            "{}: invocation_id must be set",
            case.name
        );
        assert!(
            record["timestamp"].as_str().is_some(),
            "{}: timestamp must be set",
            case.name
        );
        assert_eq!(
            record["routing_key"].as_str(),
            Some("bratch"),
            "{}: routing_key must be bratch",
            case.name
        );
        let binary_version = record["binary_version"]
            .as_str()
            .expect("binary_version must be set");
        assert!(
            semver::Version::parse(binary_version).is_ok(),
            "{}: binary_version must be valid semver, got {binary_version:?}",
            case.name
        );
        assert_eq!(
            record["corpus_version"].as_u64(),
            Some(1),
            "{}: corpus_version must be 1",
            case.name
        );
        assert!(
            record["elapsed_ms"].is_number(),
            "{}: elapsed_ms must be a number",
            case.name
        );
        assert!(
            record["additional_fields"].is_object(),
            "{}: additional_fields must be an object",
            case.name
        );
    }
}

#[test]
fn sequential_invocations_each_produce_their_own_transcript_file() {
    let dir = TempDir::new().unwrap();

    run_in_transcript_dir(&["compare", "--signature", "null-check-removed"], &dir)
        .code(ExitCode::Finding.as_i32());
    run_in_transcript_dir(&["compare", "--signature", "error-handling-narrowed"], &dir)
        .code(ExitCode::Finding.as_i32());

    let files = collect_transcript_files(&dir);
    assert_eq!(
        2,
        files.len(),
        "each sequential invocation must produce its own file"
    );

    let invocation_ids: std::collections::BTreeSet<String> = files
        .iter()
        .map(|f| {
            let record = read_transcript_record(f);
            record["invocation_id"]
                .as_str()
                .expect("invocation_id present")
                .to_owned()
        })
        .collect();
    assert_eq!(
        2,
        invocation_ids.len(),
        "sequential invocation_ids must be unique"
    );
}

#[test]
fn concurrent_invocations_produce_separate_non_overlapping_records() {
    let dir = TempDir::new().unwrap();

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let path = dir.path().to_path_buf();
            std::thread::spawn(move || {
                bratch_command()
                    .env("BSUITE_TRANSCRIPT_DIR", &path)
                    .args(["compare", "--signature", "null-check-removed"])
                    .assert()
                    .code(ExitCode::Finding.as_i32());
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("thread completed");
    }

    let files = collect_transcript_files(&dir);
    assert_eq!(
        4,
        files.len(),
        "each concurrent invocation needs its own file"
    );

    let invocation_ids: std::collections::BTreeSet<String> = files
        .iter()
        .map(|f| {
            let record = read_transcript_record(f);
            record["invocation_id"]
                .as_str()
                .expect("invocation_id present")
                .to_owned()
        })
        .collect();

    assert_eq!(4, invocation_ids.len(), "invocation_ids must be unique");
}
