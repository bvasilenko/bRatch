use assert_cmd::Command;
use bratch::RegressionSignature;
use predicates::prelude::*;

const PLACEHOLDER_DIRECTIVE_HEADER: &str = "[bratch placeholder directive - pre-corpus output]";
const ACTION_PREFIX: &str = "ACTION: This invocation reached bratch";
const EXIT_CODE_FOOTER: &str = "Exit code carries the verdict-class signal.";
const PUBLIC_INVOCATION_SURFACE_LINE: &str = "Invocation surface: cli.";
const PLACEHOLDER_SIGNATURE_VARIANT: &str = "TestAssertionWeakened";
const INTERNAL_SURFACE_TOKENS: [&str; 6] = ["L2a", "L2b", "L2c", "l2a", "l2b", "l2c"];
const COMPARE_FINDING_EXIT_CODE: i32 = 1;

#[derive(Debug, Clone, Copy)]
struct CompareCase<'a> {
    args: &'a [&'a str],
    expected_input: &'a str,
}

impl CompareCase<'_> {
    fn assert(self) {
        let stdout = compare_stdout(self.args);

        assert_compare_directive(&stdout);
        assert!(
            stdout.contains(self.expected_input),
            "stdout missing {expected:?}: {stdout}",
            expected = self.expected_input
        );
    }
}

fn bratch_command() -> Command {
    Command::cargo_bin("bratch").expect("binary exists")
}

fn command_stdout_with_code(args: &[&str], code: i32) -> String {
    let output = bratch_command()
        .args(args)
        .assert()
        .code(code)
        .get_output()
        .clone();

    String::from_utf8(output.stdout).expect("stdout is utf8")
}

fn compare_stdout(args: &[&str]) -> String {
    command_stdout_with_code(args, COMPARE_FINDING_EXIT_CODE)
}

fn successful_stdout(args: &[&str]) -> String {
    command_stdout_with_code(args, 0)
}

fn assert_usage_failure(args: &[&str], stderr_fragment: &str) {
    bratch_command()
        .args(args)
        .assert()
        .code(64)
        .stderr(predicate::str::contains(stderr_fragment));
}

fn assert_compare_directive(stdout: &str) {
    assert!(stdout.contains(PLACEHOLDER_DIRECTIVE_HEADER));
    assert!(stdout.contains("Parsed input: diff="));
    assert!(stdout.contains("history="));
    assert!(
        stdout.contains(&format!(
            "RegressionSignature::{PLACEHOLDER_SIGNATURE_VARIANT}"
        )),
        "routing key must name the specific placeholder variant {PLACEHOLDER_SIGNATURE_VARIANT:?}: {stdout}"
    );
    assert!(stdout.contains(" placeholder route."));
    assert!(stdout.contains(PUBLIC_INVOCATION_SURFACE_LINE));
    assert!(stdout.contains("Verdict-state: regression-detected."));
    assert!(stdout.contains(ACTION_PREFIX));
    assert!(stdout.contains("regression signature "));
    assert!(
        stdout.contains(PLACEHOLDER_SIGNATURE_VARIANT),
        "ACTION paragraph must name the placeholder signature variant: {stdout}"
    );
    assert!(stdout.contains(EXIT_CODE_FOOTER));
    assert!(!stdout.contains("detection behavior is deferred"));
    assert_no_internal_surface_tokens(stdout);
}

fn assert_no_internal_surface_tokens(stdout: &str) {
    for token in INTERNAL_SURFACE_TOKENS {
        assert!(!stdout.contains(token), "stdout leaked {token}: {stdout}");
    }
}

fn deferred_command_output(command_name: &str) -> String {
    format!("bratch {command_name} placeholder: behavior is deferred.\n")
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
    let stdout = successful_stdout(&["signatures"]);
    let actual = stdout.lines().collect::<Vec<_>>();
    let expected = RegressionSignature::ALL
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    assert_eq!(expected, actual);
}

#[test]
fn compare_emits_placeholder_directive_and_finding_exit_code() {
    CompareCase {
        args: &["compare", "--reason", "review requested"],
        expected_input: "diff=<none>, history=<none>",
    }
    .assert();
}

#[test]
fn compare_directive_reports_every_supported_input_combination() {
    for compare_case in [
        CompareCase {
            args: &["compare"],
            expected_input: "diff=<none>, history=<none>",
        },
        CompareCase {
            args: &["compare", "--diff", "change.diff"],
            expected_input: "diff=change.diff, history=<none>",
        },
        CompareCase {
            args: &["compare", "--history", "main"],
            expected_input: "diff=<none>, history=main",
        },
        CompareCase {
            args: &["compare", "--manifest", "manifest.json"],
            expected_input: "diff=<none>, history=<none>",
        },
        CompareCase {
            args: &["compare", "--diff", "change.diff", "--history", "main"],
            expected_input: "diff=change.diff, history=main",
        },
        CompareCase {
            args: &["compare", "--history", "v1.0.0..v2.0.0"],
            expected_input: "diff=<none>, history=v1.0.0..v2.0.0",
        },
    ] {
        compare_case.assert();
    }
}

#[test]
fn compare_quiet_and_json_flags_keep_directive_stdout() {
    for compare_case in [
        CompareCase {
            args: &["compare", "--quiet", "--reason", "review requested"],
            expected_input: "diff=<none>, history=<none>",
        },
        CompareCase {
            args: &["compare", "--json", "--reason", "review requested"],
            expected_input: "diff=<none>, history=<none>",
        },
        CompareCase {
            args: &[
                "compare",
                "--quiet",
                "--json",
                "--reason",
                "review requested",
            ],
            expected_input: "diff=<none>, history=<none>",
        },
    ] {
        compare_case.assert();
    }
}

#[test]
fn compare_rejects_blank_reason() {
    for blank_reason in ["", " ", "   ", "\t", "\n"] {
        bratch_command()
            .args(["compare", "--reason", blank_reason])
            .assert()
            .code(64)
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
        CompareCase {
            args: &["compare", "--reason", reason],
            expected_input: "diff=<none>, history=<none>",
        }
        .assert();
    }
}

#[test]
fn placeholder_commands_exit_successfully_with_stable_output() {
    for command_name in ["update", "init", "tail", "explain"] {
        let stdout = successful_stdout(&[command_name]);

        assert_eq!(deferred_command_output(command_name), stdout);
    }
}

#[test]
fn unknown_command_uses_cli_usage_failure() {
    assert_usage_failure(&["unknown"], "unrecognized subcommand");
}

#[test]
fn malformed_flag_shape_uses_cli_usage_failure() {
    for (args, stderr_fragment) in [
        (&["compare", "--reason"][..], "a value is required"),
        (&["compare", "--unknown"][..], "unexpected argument"),
        (&["compare", "--json=false"][..], "unexpected value"),
    ] {
        assert_usage_failure(args, stderr_fragment);
    }
}
