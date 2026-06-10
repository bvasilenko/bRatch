use bratch::{RegressionSignature, RevisionIdPair, VerdictClass, routing_key};
use std::str::FromStr;

struct PendingComparator;

impl PendingComparator {
    fn compare(&self) -> VerdictClass {
        unimplemented!("not yet implemented")
    }
}

#[test]
fn library_reexports_public_contract_types() {
    assert_eq!(9, RegressionSignature::ALL.len());
    assert_eq!(3, VerdictClass::ALL.len());
    let _ = RevisionIdPair::from_str("a..b").expect("RevisionIdPair is accessible");
}

#[test]
fn routing_key_uses_bratch_core_entry_point() {
    assert_eq!(bsuite_core::RoutingKey::bratch(), routing_key());
}

#[test]
#[should_panic(expected = "not yet implemented")]
fn placeholder_comparator_is_explicitly_pending() {
    let comparator = PendingComparator;

    let _ = comparator.compare();
}
