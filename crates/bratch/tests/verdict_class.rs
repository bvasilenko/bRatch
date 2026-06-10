mod common;

use bratch::VerdictClass;
use bsuite_core::ExitCode;
use common::{assert_public_name_contract, assert_rejects};
use proptest::prelude::*;
use std::str::FromStr;

proptest! {
    #[test]
    fn verdict_display_name_round_trips(index in 0..VerdictClass::ALL.len()) {
        let verdict = VerdictClass::ALL[index];
        let parsed = VerdictClass::from_str(&verdict.to_string()).expect("verdict must parse");
        prop_assert_eq!(verdict, parsed);
    }
}

#[test]
fn verdict_names_cover_exact_closed_set() {
    assert_eq!(3, VerdictClass::ALL.len());
    assert_public_name_contract(&VerdictClass::ALL);
}

#[test]
fn verdict_stable_name_matches_display_for_every_variant() {
    for variant in VerdictClass::ALL {
        assert_eq!(
            variant.stable_name(),
            variant.to_string(),
            "stable_name and Display must agree for {variant:?}"
        );
    }
}

#[test]
fn verdict_exit_codes_match_contract() {
    let cases = [
        (VerdictClass::NoRegression, ExitCode::Success, 0),
        (VerdictClass::RegressionDetected, ExitCode::Finding, 1),
        (VerdictClass::Malformed, ExitCode::InternalError, 2),
    ];

    for (verdict, expected_exit_code, expected_raw_code) in cases {
        let exit_code: ExitCode = verdict.into();

        assert_eq!(expected_exit_code, exit_code);
        assert_eq!(expected_raw_code, exit_code.as_i32());
    }
}

#[test]
fn verdict_rejects_names_outside_closed_set() {
    assert_rejects::<VerdictClass>(&[
        "unknown",
        "",
        "NoRegression",
        "RegressionDetected",
        " no-regression",
        "no-regression ",
        "no_regression",
        "clean",
        "finding",
    ]);
}
