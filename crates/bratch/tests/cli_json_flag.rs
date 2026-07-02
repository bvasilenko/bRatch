mod common;

use bratch::RegressionSignature;
use bsuite_core::ExitCode;
use common::{command_stdout, json_stdout};
use serde_json::Value;

#[test]
fn plain_output_without_json_flag_is_not_json() {
    let output = command_stdout(
        &["compare", "--signature", "null-check-removed"],
        ExitCode::Finding,
    );
    assert!(serde_json::from_str::<Value>(output.trim()).is_err());
}

#[test]
fn json_flag_produces_valid_envelope_for_all_signatures() {
    for signature in RegressionSignature::ALL {
        let envelope = json_stdout(
            &["compare", "--signature", signature.stable_name(), "--json"],
            ExitCode::Finding,
        );
        assert_eq!(envelope["schema_version"].as_u64(), Some(1));
        assert_eq!(envelope["outcome"].as_str(), Some("finding"));
        let directive = envelope["directive"].as_str().unwrap_or_default();
        let expected_prefix = format!("REGRESSION-DETECTED: {}", signature.stable_name());
        assert!(
            directive.starts_with(&expected_prefix),
            "directive for {signature} must start with {expected_prefix:?}: got {directive:?}"
        );
    }
}
