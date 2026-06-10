use serde::{Serialize, de::DeserializeOwned};
use std::{collections::BTreeSet, fmt, str::FromStr};

pub fn assert_public_name_contract<T>(values: &[T])
where
    T: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + Serialize
        + DeserializeOwned
        + Eq
        + PartialEq,
    <T as FromStr>::Err: fmt::Debug,
{
    assert_display_names_are_unique(values);
    assert_round_trip(values);
    assert_json_names_match_display(values);
    assert_rejects_surrounding_whitespace(values);
}

pub fn assert_display_names_are_unique<T: fmt::Display>(values: &[T]) {
    let names: BTreeSet<String> = values.iter().map(|v| v.to_string()).collect();

    assert_eq!(
        values.len(),
        names.len(),
        "display names are not unique: {names:?}"
    );
}

pub fn assert_round_trip<T>(values: &[T])
where
    T: Clone + Copy + fmt::Display + FromStr + Eq + PartialEq + fmt::Debug,
    <T as FromStr>::Err: fmt::Debug,
{
    for value in values {
        let name = value.to_string();
        let parsed = T::from_str(&name).expect("display name must parse");

        assert_eq!(*value, parsed, "round-trip failed for {name:?}");
    }
}

pub fn assert_rejects<T>(invalid_names: &[&str])
where
    T: FromStr,
{
    for name in invalid_names {
        assert!(
            T::from_str(name).is_err(),
            "expected rejection of {name:?} but it was accepted"
        );
    }
}

fn assert_json_names_match_display<T>(values: &[T])
where
    T: Clone + fmt::Display + Serialize + DeserializeOwned + Eq + PartialEq + fmt::Debug,
{
    for value in values {
        let encoded = serde_json::to_string(value).expect("json encode");
        let decoded: T = serde_json::from_str(&encoded).expect("json decode");
        let display_name = value.to_string();

        assert!(
            encoded.contains(&display_name),
            "json {encoded:?} does not contain display name {display_name:?}"
        );
        assert_eq!(*value, decoded, "json round-trip failed");
    }
}

fn assert_rejects_surrounding_whitespace<T>(values: &[T])
where
    T: fmt::Display + FromStr,
{
    for value in values {
        let display_name = value.to_string();

        assert!(
            T::from_str(&format!(" {display_name}")).is_err(),
            "leading whitespace should be rejected for {display_name:?}"
        );
        assert!(
            T::from_str(&format!("{display_name} ")).is_err(),
            "trailing whitespace should be rejected for {display_name:?}"
        );
    }
}
