use bratch::{
    BratchError,
    substrate_input::{RevisionIdPair, SubstrateInput},
};
use proptest::prelude::*;
use std::{path::PathBuf, str::FromStr};

proptest! {
    #[test]
    fn revision_id_pair_round_trips_for_arbitrary_valid_refs(
        baseline in "[a-zA-Z0-9_/-]{1,32}",
        candidate in "[a-zA-Z0-9_/-]{1,32}",
    ) {
        let input = format!("{baseline}..{candidate}");
        let pair = RevisionIdPair::from_str(&input).expect("valid pair");
        prop_assert_eq!(&baseline, &pair.baseline.0);
        prop_assert_eq!(&candidate, &pair.candidate.0);
        prop_assert_eq!(input, pair.to_string());
    }
}

#[test]
fn revision_id_pair_display_round_trips() {
    let pair = RevisionIdPair::from_str("abc123..def456").expect("valid pair");

    assert_eq!("abc123", pair.baseline.0);
    assert_eq!("def456", pair.candidate.0);
    assert_eq!("abc123..def456", pair.to_string());
}

#[test]
fn revision_id_pair_rejects_malformed_inputs() {
    for malformed in ["", "abc", "abc..def..ghi", "..def", "abc..", ".."] {
        assert!(
            RevisionIdPair::from_str(malformed).is_err(),
            "expected rejection of {malformed:?} but it was accepted"
        );
    }
}

#[test]
fn revision_id_pair_accepts_various_ref_formats() {
    for valid in [
        "main..feature-branch",
        "v1.0.0..v2.0.0",
        "abc123..def456",
        "HEAD~3..HEAD",
    ] {
        assert!(
            RevisionIdPair::from_str(valid).is_ok(),
            "expected acceptance of {valid:?} but it was rejected"
        );
    }
}

#[test]
fn substrate_input_accepts_every_supported_presence_combination() {
    for (diff, history) in [
        (None, None),
        (Some(PathBuf::from("change.diff")), None),
        (None, Some("main".to_owned())),
        (Some(PathBuf::from("change.diff")), Some("main".to_owned())),
    ] {
        let input = SubstrateInput::new(diff.clone(), history.clone()).expect("input is valid");

        assert_eq!(diff, input.diff);
        assert_eq!(history, input.history);
    }
}

#[test]
fn substrate_input_rejects_empty_diff_path() {
    let error =
        SubstrateInput::new(Some(PathBuf::from("")), None).expect_err("empty diff path rejected");

    assert!(matches!(error, BratchError::DiffSliceInvalid(_)));
}

#[test]
fn substrate_input_rejects_empty_history_ref() {
    let error =
        SubstrateInput::new(None, Some(String::new())).expect_err("empty history ref rejected");

    assert!(matches!(error, BratchError::HistoryRefInvalid(_)));
}
