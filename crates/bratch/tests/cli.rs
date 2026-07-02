mod common;

use bratch::RegressionSignature;
use bsuite_core::ExitCode;
use common::{bratch_command, command_stdout, compare_stdout};
use predicates::prelude::*;

const CORPUS_DIRECTIVE_PREFIX: &str = "REGRESSION-DETECTED:";
const INTERNAL_SURFACE_TOKENS: [&str; 6] = ["L2a", "L2b", "L2c", "l2a", "l2b", "l2c"];

fn assert_usage_failure(args: &[&str], stderr_fragment: &str) {
    bratch_command()
        .args(args)
        .assert()
        .code(ExitCode::Usage.as_i32())
        .stderr(predicate::str::contains(stderr_fragment));
}

fn assert_corpus_directive(stdout: &str, signature: &str) {
    assert!(
        stdout.contains(CORPUS_DIRECTIVE_PREFIX),
        "directive must contain {CORPUS_DIRECTIVE_PREFIX:?}: {stdout}"
    );
    assert!(
        stdout.contains(signature),
        "directive must name {signature:?}: {stdout}"
    );
    assert!(
        !stdout.contains("placeholder directive"),
        "corpus-backed directive must not reference placeholder: {stdout}"
    );
    for token in INTERNAL_SURFACE_TOKENS {
        assert!(!stdout.contains(token), "stdout leaked {token}: {stdout}");
    }
}

#[test]
fn help_exits_successfully() {
    bratch_command()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "CLI regression-signature detector",
        ));
}

#[test]
fn signatures_exits_successfully_and_prints_exact_closed_set() {
    let stdout = command_stdout(&["signatures"], ExitCode::Success);
    let actual = stdout.lines().collect::<Vec<_>>();
    let expected = RegressionSignature::ALL
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    assert_eq!(expected, actual);
}

#[test]
fn compare_emits_distinct_directive_for_every_signature() {
    for signature in RegressionSignature::ALL {
        let stdout = compare_stdout(signature.stable_name(), &[]);
        assert_corpus_directive(&stdout, signature.stable_name());
    }
}

#[test]
fn compare_accepts_supported_input_flags() {
    for extra_args in [
        &[][..],
        &["--diff", "change.diff"][..],
        &["--history", "main"][..],
        &["--manifest", "manifest.json"][..],
        &["--diff", "change.diff", "--history", "main"][..],
    ] {
        let stdout = compare_stdout("null-check-removed", extra_args);
        assert_corpus_directive(&stdout, "null-check-removed");
    }
}

#[test]
fn compare_quiet_and_json_flags_keep_directive_stdout() {
    for extra_args in [
        &["--quiet"][..],
        &["--json"][..],
        &["--quiet", "--json"][..],
    ] {
        let stdout = compare_stdout("null-check-removed", extra_args);
        assert!(
            stdout.contains("null-check-removed"),
            "directive must name signature: {stdout}"
        );
    }
}

#[test]
fn compare_rejects_blank_reason() {
    for blank_reason in ["", " ", "   ", "\t", "\n"] {
        bratch_command()
            .args([
                "compare",
                "--signature",
                "null-check-removed",
                "--reason",
                blank_reason,
            ])
            .assert()
            .code(ExitCode::Usage.as_i32())
            .stderr(predicate::str::contains("reason must not be empty"));
    }
}

#[test]
fn compare_accepts_every_non_blank_reason_shape() {
    for reason in [
        "review requested",
        " review requested ",
        "review\trequested",
    ] {
        let stdout = compare_stdout("null-check-removed", &["--reason", reason]);
        assert_corpus_directive(&stdout, "null-check-removed");
    }
}

#[test]
fn compare_rejects_unknown_signature() {
    assert_usage_failure(
        &["compare", "--signature", "unknown-signature"],
        "unknown regression signature",
    );
}

#[test]
fn compare_rejects_missing_or_blank_signature() {
    for (args, stderr_fragment) in [
        (&["compare"][..], "required arguments were not provided"),
        (
            &["compare", "--signature", ""][..],
            "unknown regression signature",
        ),
        (
            &["compare", "--signature", " null-check-removed"][..],
            "unknown regression signature",
        ),
        (
            &["compare", "--signature", "null-check-removed "][..],
            "unknown regression signature",
        ),
    ] {
        assert_usage_failure(args, stderr_fragment);
    }
}

#[test]
fn compare_json_flag_with_malformed_signature_routes_to_stderr() {
    bratch_command()
        .args(["compare", "--signature", "not-a-valid-type", "--json"])
        .assert()
        .code(ExitCode::Usage.as_i32())
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("unknown regression signature"));
}

#[test]
fn unknown_command_uses_cli_usage_failure() {
    assert_usage_failure(&["unknown"], "unrecognized subcommand");
}

#[test]
fn malformed_flag_shape_uses_cli_usage_failure() {
    for (args, stderr_fragment) in [
        (
            &["compare", "--signature", "null-check-removed", "--reason"][..],
            "a value is required",
        ),
        (
            &["compare", "--signature", "null-check-removed", "--unknown"][..],
            "unexpected argument",
        ),
        (
            &[
                "compare",
                "--signature",
                "null-check-removed",
                "--json=false",
            ][..],
            "unexpected value",
        ),
    ] {
        assert_usage_failure(args, stderr_fragment);
    }
}
