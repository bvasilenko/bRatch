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
    for error in [
        BratchError::Usage("bad arguments".to_owned()),
        BratchError::DiffSliceInvalid("bad diff".to_owned()),
        BratchError::HistoryRefInvalid("bad history".to_owned()),
        BratchError::TaxonomyUnknown("bad signature".to_owned()),
    ] {
        assert_error_exit_code(error, ExitCode::Usage);
    }
}

#[test]
fn is_malformed_input_agrees_with_exit_code_classification() {
    for error in [
        BratchError::Usage("u".to_owned()),
        BratchError::DiffSliceInvalid("d".to_owned()),
        BratchError::HistoryRefInvalid("h".to_owned()),
        BratchError::TaxonomyUnknown("t".to_owned()),
    ] {
        assert!(error.is_malformed_input(), "must be malformed: {error}");
        assert_eq!(
            error.exit_code(),
            ExitCode::Usage,
            "malformed input must map to Usage: {error}"
        );
    }
    for error in [
        BratchError::VerdictClassUnknown("v".to_owned()),
        BratchError::RevisionIdPairMalformed("r".to_owned()),
        BratchError::InvocationSurfaceUnknown("i".to_owned()),
        BratchError::CorpusLoad("c".to_owned()),
        BratchError::Core(BsuiteCoreError::PromptResolution("p".to_owned())),
        BratchError::Core(BsuiteCoreError::CorpusSignatureInvalid),
        BratchError::Core(BsuiteCoreError::CorpusDeserializationFailed(
            "bad corpus".to_owned(),
        )),
    ] {
        assert!(
            !error.is_malformed_input(),
            "must not be malformed: {error}"
        );
        assert_eq!(
            error.exit_code(),
            ExitCode::InternalError,
            "domain error must map to InternalError: {error}"
        );
    }
}

#[test]
fn domain_errors_map_to_internal_error_exit_code() {
    for error in [
        BratchError::VerdictClassUnknown("bad verdict".to_owned()),
        BratchError::RevisionIdPairMalformed("bad pair".to_owned()),
        BratchError::InvocationSurfaceUnknown("bad surface".to_owned()),
        BratchError::CorpusLoad("bad corpus".to_owned()),
        BratchError::Core(BsuiteCoreError::PromptResolution("bad prompt".to_owned())),
        BratchError::Core(BsuiteCoreError::Update("bad update".to_owned())),
        BratchError::Core(BsuiteCoreError::Transcript("bad transcript".to_owned())),
        BratchError::Core(BsuiteCoreError::ManifestOverlay(
            OverlayValidationError::SignatureMissing,
        )),
        BratchError::Core(BsuiteCoreError::ExitCode("bad exit".to_owned())),
        BratchError::Core(BsuiteCoreError::VisibilityEvidence(
            "bad evidence".to_owned(),
        )),
        BratchError::Core(BsuiteCoreError::AdapterHostBinding("bad bind".to_owned())),
        BratchError::Core(BsuiteCoreError::CorpusSignatureInvalid),
        BratchError::Core(BsuiteCoreError::CorpusDeserializationFailed(
            "bad corpus".to_owned(),
        )),
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
        BratchError::CorpusLoad("bad corpus".to_owned()),
        BratchError::Core(BsuiteCoreError::PromptResolution("bad prompt".to_owned())),
        BratchError::Core(BsuiteCoreError::Update("bad update".to_owned())),
        BratchError::Core(BsuiteCoreError::Transcript("bad transcript".to_owned())),
        BratchError::Core(BsuiteCoreError::ManifestOverlay(
            OverlayValidationError::SignatureMissing,
        )),
        BratchError::Core(BsuiteCoreError::ExitCode("bad exit".to_owned())),
        BratchError::Core(BsuiteCoreError::CorpusSignatureInvalid),
        BratchError::Core(BsuiteCoreError::CorpusDeserializationFailed(
            "bad corpus".to_owned(),
        )),
    ] {
        let message = error.to_string();
        assert!(!message.is_empty(), "error message must not be empty");
    }
}

#[test]
fn string_wrapping_variants_preserve_message_text() {
    let message = "caller-supplied reason";
    assert_eq!(message, BratchError::Usage(message.to_owned()).to_string());
    for error in [
        BratchError::DiffSliceInvalid(message.to_owned()),
        BratchError::HistoryRefInvalid(message.to_owned()),
        BratchError::TaxonomyUnknown(message.to_owned()),
        BratchError::VerdictClassUnknown(message.to_owned()),
        BratchError::RevisionIdPairMalformed(message.to_owned()),
        BratchError::InvocationSurfaceUnknown(message.to_owned()),
        BratchError::CorpusLoad(message.to_owned()),
    ] {
        assert!(
            error.to_string().contains(message),
            "Display must include message text: {error}"
        );
    }
}

#[test]
fn corpus_load_error_converts_to_core_via_into_core() {
    let error = BratchError::CorpusLoad("parse failure".to_owned());
    let core = error.into_core();
    assert!(matches!(
        core,
        BsuiteCoreError::CorpusDeserializationFailed(_)
    ));
}

#[test]
fn core_passthrough_variant_converts_to_identical_core_error() {
    let inner = BsuiteCoreError::Update("network timeout".to_owned());
    let error = BratchError::Core(inner);
    let core = error.into_core();
    assert!(
        matches!(core, BsuiteCoreError::Update(_)),
        "Core passthrough must yield the same BsuiteCoreError variant: {core:?}"
    );
}

#[test]
fn non_corpus_non_core_variants_convert_to_prompt_resolution_error() {
    for error in [
        BratchError::TaxonomyUnknown("x".to_owned()),
        BratchError::VerdictClassUnknown("x".to_owned()),
        BratchError::RevisionIdPairMalformed("x".to_owned()),
        BratchError::InvocationSurfaceUnknown("x".to_owned()),
        BratchError::Usage("x".to_owned()),
        BratchError::DiffSliceInvalid("x".to_owned()),
        BratchError::HistoryRefInvalid("x".to_owned()),
    ] {
        let display = error.to_string();
        let core = error.into_core();
        match core {
            BsuiteCoreError::PromptResolution(text) => {
                assert!(
                    !text.is_empty(),
                    "PromptResolution payload must be non-empty for {display:?}"
                );
            }
            other => panic!("expected PromptResolution for {display:?}, got {other:?}"),
        }
    }
}
