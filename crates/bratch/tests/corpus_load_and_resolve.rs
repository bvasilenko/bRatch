use bratch::{BratchError, RegressionSignature, corpus_index::RegressionSignatureCorpusIndex};
use bsuite_core::BsuiteCoreError;
use ed25519_dalek::{SigningKey, VerifyingKey};
use std::collections::BTreeSet;

const CORPUS_TOML: &str = include_str!("../corpus/bratch-v0.toml");
const PUBKEY_BYTES: &[u8] = include_bytes!("../corpus/bratch-v0-pubkey.bin");

fn load_verifying_key() -> VerifyingKey {
    let bytes: [u8; 32] = PUBKEY_BYTES.try_into().expect("pubkey is 32 bytes");
    VerifyingKey::from_bytes(&bytes).expect("pubkey is valid")
}

fn load_corpus() -> RegressionSignatureCorpusIndex {
    RegressionSignatureCorpusIndex::from_toml_signed(CORPUS_TOML, &load_verifying_key())
        .expect("fixture corpus loads cleanly")
}

fn remove_entry(corpus: &str, signature: &str) -> String {
    let mut sections = corpus.split("\n[[entries]]");
    let preamble = sections.next().expect("corpus has preamble");
    let mut output = preamble.to_owned();
    for section in sections {
        if !section.contains(&format!("regression_signature = \"{signature}\"")) {
            output.push_str("\n[[entries]]");
            output.push_str(section);
        }
    }
    output
}

#[test]
fn all_regression_signatures_are_indexed_with_distinct_non_empty_directives() {
    let corpus = load_corpus();
    let mut seen = BTreeSet::new();
    for signature in RegressionSignature::ALL {
        let directive = corpus.resolve(signature);
        assert!(
            !directive.as_str().is_empty(),
            "empty directive for {signature}"
        );
        assert!(
            seen.insert(directive.as_str().to_owned()),
            "duplicate directive for {signature}"
        );
    }
    assert_eq!(RegressionSignature::ALL.len(), seen.len());
}

#[test]
fn corpus_rejects_wrong_pubkey() {
    let wrong_seed = [0x00u8; 32];
    let wrong_pubkey = SigningKey::from_bytes(&wrong_seed).verifying_key();
    let result = RegressionSignatureCorpusIndex::from_toml_signed(CORPUS_TOML, &wrong_pubkey);
    assert!(result.is_err(), "wrong-pubkey corpus must be rejected");
    assert!(
        matches!(
            result.unwrap_err(),
            BratchError::Core(BsuiteCoreError::CorpusSignatureInvalid)
        ),
        "error kind must be CorpusSignatureInvalid"
    );
}

#[test]
fn corpus_rejects_tampered_directive_content() {
    let corpus = load_corpus();
    let first_directive = corpus
        .resolve(RegressionSignature::ALL[0])
        .as_str()
        .to_owned();
    let first_word = first_directive
        .split_whitespace()
        .next()
        .expect("directive has at least one word");

    let tampered = CORPUS_TOML.replacen(first_word, "TAMPERED", 1);
    let result = RegressionSignatureCorpusIndex::from_toml_signed(&tampered, &load_verifying_key());
    assert!(result.is_err(), "tampered directive must be rejected");
    assert!(
        matches!(
            result.unwrap_err(),
            BratchError::Core(BsuiteCoreError::CorpusSignatureInvalid)
        ),
        "tamper detection must be CorpusSignatureInvalid"
    );
}

#[test]
fn corpus_index_rejects_duplicate_regression_signature() {
    let first = RegressionSignature::ALL[0].stable_name();
    let second = RegressionSignature::ALL[1].stable_name();
    let with_duplicate = CORPUS_TOML.replacen(
        &format!("regression_signature = \"{second}\""),
        &format!("regression_signature = \"{first}\""),
        1,
    );
    let result =
        RegressionSignatureCorpusIndex::from_toml_signed(&with_duplicate, &load_verifying_key());
    assert!(result.is_err(), "duplicate entry must be rejected");
    assert!(matches!(result.unwrap_err(), BratchError::CorpusLoad(_)));
}

#[test]
fn corpus_index_rejects_unknown_regression_signature_string() {
    let invalid_names = [
        "not-a-valid-type",
        "",
        "NullCheckRemoved",
        " null-check-removed",
        "null-check-removed ",
        "null_check_removed",
        "42",
        "null-check",
    ];
    for name in invalid_names {
        let mutated = CORPUS_TOML.replacen(
            &format!(
                "regression_signature = \"{}\"",
                RegressionSignature::ALL[0].stable_name()
            ),
            &format!("regression_signature = \"{name}\""),
            1,
        );
        let result =
            RegressionSignatureCorpusIndex::from_toml_signed(&mutated, &load_verifying_key());
        assert!(result.is_err(), "invalid name {name:?} must be rejected");
    }
}

#[test]
fn corpus_index_rejects_every_individually_missing_signature() {
    for signature in RegressionSignature::ALL {
        let without = remove_entry(CORPUS_TOML, signature.stable_name());
        let result =
            RegressionSignatureCorpusIndex::from_toml_signed(&without, &load_verifying_key());
        assert!(
            result.is_err(),
            "corpus missing {signature} must be rejected"
        );
        assert!(
            matches!(
                result.unwrap_err(),
                BratchError::Core(BsuiteCoreError::CorpusSignatureInvalid)
            ),
            "removing a signed entry must fail at signature verification"
        );
    }
}

#[test]
fn wrong_pubkey_error_converts_to_corpus_error_via_into_core() {
    let wrong_seed = [0x00u8; 32];
    let wrong_pubkey = SigningKey::from_bytes(&wrong_seed).verifying_key();
    let result = RegressionSignatureCorpusIndex::from_toml_signed(CORPUS_TOML, &wrong_pubkey);
    let err = result.expect_err("wrong pubkey must be rejected");
    let core = err.into_core();
    assert!(
        matches!(
            core,
            BsuiteCoreError::CorpusSignatureInvalid
                | BsuiteCoreError::CorpusDeserializationFailed(_)
        ),
        "into_core must yield a corpus-related error variant: {core:?}"
    );
}

#[test]
fn corpus_rejects_unsigned_appended_entry() {
    let extra = "\n[[entries]]\nrouting_key = \"bratch\"\nregression_signature = \"null-check-removed\"\ndirective = \"Extra.\"\n[entries.provenance]\nrun_id = \"x\"\niteration = 0\nobservation_source = \"x\"\npre_compliance = 0.0\npost_compliance = 0.0\n";
    let with_extra = format!("{CORPUS_TOML}{extra}");
    let result =
        RegressionSignatureCorpusIndex::from_toml_signed(&with_extra, &load_verifying_key());
    assert!(result.is_err(), "unsigned appended entry must be rejected");
    assert!(matches!(
        result.unwrap_err(),
        BratchError::Core(BsuiteCoreError::CorpusSignatureInvalid)
    ));
}
