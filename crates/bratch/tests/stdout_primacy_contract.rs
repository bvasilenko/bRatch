mod common;

use bratch::RegressionSignature;
use bsuite_core::ExitCode;
use common::bratch_command;
use tempfile::TempDir;

#[test]
fn compare_routes_directive_to_stdout_and_nothing_to_stderr_for_all_signatures() {
    let transcript_dir = TempDir::new().unwrap();
    for signature in RegressionSignature::ALL {
        let output = bratch_command()
            .env("BSUITE_TRANSCRIPT_DIR", transcript_dir.path())
            .args(["compare", "--signature", signature.stable_name()])
            .assert()
            .code(ExitCode::Finding.as_i32())
            .get_output()
            .clone();

        assert!(!output.stdout.is_empty(), "{signature}: empty stdout");
        assert!(
            output.stderr.is_empty(),
            "{signature}: stderr was {:?}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn malformed_signature_routes_error_to_stderr_and_nothing_to_stdout() {
    let output = bratch_command()
        .args(["compare", "--signature", "unknown-signature"])
        .assert()
        .code(ExitCode::Usage.as_i32())
        .get_output()
        .clone();

    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
}

#[test]
fn deferred_verbs_produce_no_output_on_either_stream_and_exit_successfully() {
    let transcript_dir = TempDir::new().unwrap();
    for subcmd in ["init", "tail", "explain"] {
        let output = bratch_command()
            .env("BSUITE_TRANSCRIPT_DIR", transcript_dir.path())
            .arg(subcmd)
            .assert()
            .code(ExitCode::Success.as_i32())
            .get_output()
            .clone();

        assert!(output.stdout.is_empty(), "{subcmd}: stdout not empty");
        assert!(output.stderr.is_empty(), "{subcmd}: stderr not empty");
    }
}

#[test]
fn signatures_listing_goes_to_stdout_not_stderr_and_is_not_json() {
    let transcript_dir = TempDir::new().unwrap();
    let output = bratch_command()
        .env("BSUITE_TRANSCRIPT_DIR", transcript_dir.path())
        .arg("signatures")
        .assert()
        .code(ExitCode::Success.as_i32())
        .get_output()
        .clone();

    let stdout = String::from_utf8(output.stdout).expect("signatures stdout is UTF-8");
    assert!(!stdout.is_empty());
    assert!(output.stderr.is_empty());
    assert!(serde_json::from_str::<serde_json::Value>(stdout.trim()).is_err());
}
