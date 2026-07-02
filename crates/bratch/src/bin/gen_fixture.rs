use base64::Engine;
use bsuite_core::{CorpusEntry, CorpusFile, ProvenanceRecord, RoutingKey};
use ed25519_dalek::{Signer, SigningKey};
use serde::Serialize;
use std::{fs, path::Path};

const FIXTURE_SEED: [u8; 32] = *b"bratch-fixture-v0-seed-000000000";
const CANONICAL_KEY_ID: &str = "bratch-fixture-v0";
const OBSERVATION_SOURCE: &str = "hand-authored-fixture-v0";
const RUN_ID: &str = "hand-authored-fixture-v0";

struct EntrySpec {
    regression_signature: &'static str,
    directive: &'static str,
}

const ENTRIES: &[EntrySpec] = &[
    EntrySpec {
        regression_signature: "null-check-removed",
        directive: "REGRESSION-DETECTED: null-check-removed. Locate the removed guard and determine what values were previously rejected at that boundary. Run or reproduce the code path with a null or absent value and observe whether a panic, undefined behavior, or incorrect output now occurs. If the guard was protecting a downstream caller, trace that caller's assumption about the value's presence before accepting the removal. Restore the guard and address the root cause if the downstream invariant no longer holds.",
    },
    EntrySpec {
        regression_signature: "error-handling-narrowed",
        directive: "REGRESSION-DETECTED: error-handling-narrowed. Compare the error surface before and after the change and identify every error kind that is no longer explicitly handled. Verify whether each dropped error kind can still be reached in the execution path by tracing the call graph from its origin. If a dropped case is reachable at runtime, restore the handler or provide explicit evidence that the error is now prevented upstream rather than silently swallowed. Do not claim the narrowing is safe without observable proof of the prevention path.",
    },
    EntrySpec {
        regression_signature: "test-assertion-weakened",
        directive: "REGRESSION-DETECTED: test-assertion-weakened. State the behavioral contract the original assertion was verifying and confirm whether the weakened form still enforces that contract. Run the test with a known-bad input that the original assertion would have caught and observe whether the weakened form still rejects it. If the weakened assertion passes on an input that the original would have caught, restore the original assertion and fix the underlying implementation instead. A green test on a diluted assertion is not evidence of correct behavior.",
    },
    EntrySpec {
        regression_signature: "scope-boundary-broken",
        directive: "REGRESSION-DETECTED: scope-boundary-broken. Identify the module, package, or visibility boundary that has been crossed and name the invariant that boundary was enforcing. Determine which callers now have direct access to implementation details they previously could not reach, and trace whether any of those callers rely on the exposed internals in ways that bypass validation. If the coupling is unintended, restore the boundary and route the access through the existing public interface or an explicitly designed extension point. Document any intentional boundary change with a rationale before proceeding.",
    },
    EntrySpec {
        regression_signature: "invariant-violated",
        directive: "REGRESSION-DETECTED: invariant-violated. Identify the specific invariant that the change violates and find the location where that invariant is established in the codebase. Determine whether the data still satisfies the invariant after the change by running or inspecting the affected operation boundary with concrete values. If the invariant is violated, either restore the condition that establishes it or provide written evidence that the invariant no longer applies to the current design and update the documentation accordingly. Do not proceed while any consumer of the invariant still depends on the old guarantee.",
    },
    EntrySpec {
        regression_signature: "brand-voice-loosened",
        directive: "REGRESSION-DETECTED: brand-voice-loosened. Compare the changed passage against the approved voice specification and identify the specific dimension that has drifted, such as tone, formality level, sentence length, or prohibited phrasing. Locate the voice guideline that governs the affected passage and determine whether an explicit editorial exception was authorized by the content owner. If the drift is unauthorized, restore the approved voice for that passage before advancing the content change. Record the exception with an approver name and date if brand-voice relaxation is intentional.",
    },
    EntrySpec {
        regression_signature: "banned-term-re-introduced",
        directive: "REGRESSION-DETECTED: banned-term-re-introduced. Identify the exact term or phrase that appears in the banned list and locate every occurrence in the changed content. Check whether the term carries a specific prohibited meaning in the current regulatory, brand, or product context that any proposed synonym does not. Remove or replace every occurrence with an approved alternative, then confirm that the replacement preserves required meaning and does not introduce a new compliance gap. Do not submit the content until each occurrence is resolved and documented.",
    },
    EntrySpec {
        regression_signature: "disclosure-line-deleted",
        directive: "REGRESSION-DETECTED: disclosure-line-deleted. Identify the disclosure statement that was removed and determine whether the regulatory, brand, or contractual requirement that mandates it still applies to this content type and jurisdiction. If the requirement still applies, restore the disclosure at the correct position in the document and verify that the surrounding content does not render the disclosure ambiguous or incomplete. Obtain an explicit exception record citing the authorizing party before accepting any removal. Never omit a required disclosure without documented approval.",
    },
    EntrySpec {
        regression_signature: "approved-fact-replaced",
        directive: "REGRESSION-DETECTED: approved-fact-replaced. Compare the replacement statement against the originally approved fact and determine whether the factual authority, accuracy, or permitted scope has changed. Identify the source document, review record, or stakeholder approval that granted the original statement its approved status. If the replacement has not completed the same approval process, revert to the approved fact and route the proposed change through the appropriate review channel before using it in published content. Do not treat editorial rephrasing as equivalent to fact approval.",
    },
];

#[derive(Serialize)]
struct FixtureFile {
    schema_version: u32,
    signature: String,
    canonical_key_id: &'static str,
    entries: Vec<FixtureEntry>,
}

#[derive(Serialize)]
struct FixtureEntry {
    routing_key: RoutingKey,
    regression_signature: &'static str,
    directive: &'static str,
    provenance: FixtureProvenance,
}

#[derive(Serialize)]
struct FixtureProvenance {
    run_id: &'static str,
    iteration: u32,
    observation_source: &'static str,
    pre_compliance: f64,
    post_compliance: f64,
}

fn main() {
    let signing_key = SigningKey::from_bytes(&FIXTURE_SEED);
    let verifying_key = signing_key.verifying_key();

    let mut corpus = CorpusFile {
        schema_version: 1,
        signature: String::new(),
        canonical_key_id: CANONICAL_KEY_ID.to_owned(),
        entries: ENTRIES
            .iter()
            .map(|spec| CorpusEntry {
                routing_key: RoutingKey::BRatch,
                directive: spec.directive.to_owned(),
                provenance: ProvenanceRecord {
                    run_id: RUN_ID.to_owned(),
                    iteration: 0,
                    observation_source: OBSERVATION_SOURCE.to_owned(),
                    pre_compliance: 0.0,
                    post_compliance: 0.0,
                },
            })
            .collect(),
    };

    let payload_bytes = bsuite_core::corpus::canonical_payload_bytes(&corpus)
        .expect("fixture corpus canonicalization succeeds");
    let signature = signing_key.sign(&payload_bytes);
    let sig_b64 = base64::engine::general_purpose::STANDARD.encode(signature.to_bytes());
    corpus.signature = format!("ed25519:{sig_b64}");

    let fixture_file = FixtureFile {
        schema_version: 1,
        signature: corpus.signature,
        canonical_key_id: CANONICAL_KEY_ID,
        entries: ENTRIES
            .iter()
            .map(|spec| FixtureEntry {
                routing_key: RoutingKey::BRatch,
                regression_signature: spec.regression_signature,
                directive: spec.directive,
                provenance: FixtureProvenance {
                    run_id: RUN_ID,
                    iteration: 0,
                    observation_source: OBSERVATION_SOURCE,
                    pre_compliance: 0.0,
                    post_compliance: 0.0,
                },
            })
            .collect(),
    };

    let header = "# Fixture corpus. Hand-authored seed material until an evolved corpus ships at a later cycle. Not for production trust.\n\n";
    let body = toml::to_string_pretty(&fixture_file).expect("fixture TOML serialization succeeds");
    let out_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus");
    fs::create_dir_all(&out_dir).expect("create corpus directory");
    fs::write(out_dir.join("bratch-v0.toml"), format!("{header}{body}"))
        .expect("write corpus TOML");
    fs::write(
        out_dir.join("bratch-v0-pubkey.bin"),
        verifying_key.to_bytes(),
    )
    .expect("write verifying key");
    fs::write(
        out_dir.join("bratch-v0-signkey.bin"),
        signing_key.to_bytes(),
    )
    .expect("write signing key");

    println!("corpus files written to {}", out_dir.display());
}
