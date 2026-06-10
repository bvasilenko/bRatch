use bratch::BratchError;
use bratch::error::process_exit_code;
use bsuite_core::{BsuiteCoreError, ExitCode, OverlayValidationError};

fn assert_error_exit_code(error: BratchError, expected: ExitCode) {
    assert_eq!(expected, error.exit_code());
    assert_eq!(
        std::process::ExitCode::from(expected.as_i32() as u8),
        error.process_exit_code()
    );
}

#[test]
fn usage_errors_map_to_usage_exit_code() {
    assert_error_exit_code(
        BratchError::Usage("bad arguments".to_owned()),
        ExitCode::Usage,
    );
}

#[test]
fn domain_errors_map_to_internal_error_exit_code() {
    for error in [
        BratchError::DiffSliceInvalid("bad diff".to_owned()),
        BratchError::HistoryRefInvalid("bad history".to_owned()),
        BratchError::TaxonomyUnknown("bad signature".to_owned()),
        BratchError::VerdictClassUnknown("bad verdict".to_owned()),
        BratchError::RevisionIdPairMalformed("bad pair".to_owned()),
        BratchError::InvocationSurfaceUnknown("bad surface".to_owned()),
        BratchError::Core(BsuiteCoreError::PromptResolution("bad prompt".to_owned())),
        BratchError::Core(BsuiteCoreError::Update("bad update".to_owned())),
        BratchError::Core(BsuiteCoreError::Transcript("bad transcript".to_owned())),
        BratchError::Core(BsuiteCoreError::ManifestOverlay(
            OverlayValidationError::EmptyKey,
        )),
        BratchError::Core(BsuiteCoreError::ExitCode("bad exit".to_owned())),
        BratchError::Core(BsuiteCoreError::VisibilityEvidence(
            "bad evidence".to_owned(),
        )),
        BratchError::Core(BsuiteCoreError::AdapterHostBinding("bad bind".to_owned())),
    ] {
        assert_error_exit_code(error, ExitCode::InternalError);
    }
}

#[test]
fn process_exit_code_maps_all_exit_code_variants_to_correct_raw_bytes() {
    let cases: &[(ExitCode, u8)] = &[
        (ExitCode::Success, 0),
        (ExitCode::Finding, 1),
        (ExitCode::InternalError, 2),
        (ExitCode::Usage, 64),
    ];

    for &(code, expected_raw) in cases {
        assert_eq!(
            std::process::ExitCode::from(expected_raw),
            process_exit_code(code),
            "ExitCode::{code:?} must produce raw byte {expected_raw}"
        );
    }
}

#[test]
fn error_display_messages_are_non_empty() {
    for error in [
        BratchError::Usage("bad arguments".to_owned()),
        BratchError::DiffSliceInvalid("bad diff".to_owned()),
        BratchError::HistoryRefInvalid("bad history".to_owned()),
        BratchError::TaxonomyUnknown("bad signature".to_owned()),
        BratchError::VerdictClassUnknown("bad verdict".to_owned()),
        BratchError::RevisionIdPairMalformed("bad pair".to_owned()),
        BratchError::InvocationSurfaceUnknown("bad surface".to_owned()),
        BratchError::Core(BsuiteCoreError::PromptResolution("bad prompt".to_owned())),
    ] {
        let message = error.to_string();

        assert!(!message.is_empty(), "error message must not be empty");
    }
}

#[test]
fn usage_error_preserves_message_text() {
    let message = "caller-supplied reason";
    let error = BratchError::Usage(message.to_owned());

    assert_eq!(message, error.to_string());
}
