mod common;

use bratch::RegressionSignature;
use common::{assert_public_name_contract, assert_rejects};
use proptest::prelude::*;
use std::str::FromStr;

proptest! {
    #[test]
    fn signature_display_name_round_trips(index in 0..RegressionSignature::ALL.len()) {
        let signature = RegressionSignature::ALL[index];
        let parsed = RegressionSignature::from_str(&signature.to_string())
            .expect("signature must parse");
        prop_assert_eq!(signature, parsed);
    }
}

#[test]
fn signature_names_cover_exact_closed_set() {
    assert_eq!(9, RegressionSignature::ALL.len());
    assert_public_name_contract(&RegressionSignature::ALL);
}

#[test]
fn signature_stable_name_matches_display_for_every_variant() {
    for variant in RegressionSignature::ALL {
        assert_eq!(
            variant.stable_name(),
            variant.to_string(),
            "stable_name and Display must agree for {variant:?}"
        );
    }
}

#[test]
fn signature_rejects_names_outside_closed_set() {
    assert_rejects::<RegressionSignature>(&[
        "unknown",
        "",
        "TestAssertionWeakened",
        " test-assertion-weakened",
        "test-assertion-weakened ",
        "test_assertion_weakened",
        "null-check",
        "regression",
        "brand-voice",
    ]);
}
