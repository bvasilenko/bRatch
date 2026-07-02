use crate::{BratchError, RegressionSignature};
use bsuite_core::{corpus::parse_signed_corpus, prompt_resolver::DirectiveString};
use ed25519_dalek::VerifyingKey;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct ExtendedCorpusFile {
    entries: Vec<ExtendedCorpusEntry>,
}

#[derive(Deserialize)]
struct ExtendedCorpusEntry {
    regression_signature: String,
    directive: String,
}

#[derive(Debug)]
pub struct RegressionSignatureCorpusIndex {
    by_signature: HashMap<RegressionSignature, DirectiveString>,
}

impl RegressionSignatureCorpusIndex {
    pub fn from_toml_signed(corpus_toml: &str, pubkey: &VerifyingKey) -> Result<Self, BratchError> {
        parse_signed_corpus(corpus_toml, pubkey).map_err(BratchError::Core)?;
        let extended: ExtendedCorpusFile =
            toml::from_str(corpus_toml).map_err(|e| BratchError::CorpusLoad(e.to_string()))?;
        Self::build_index(extended.entries)
    }

    fn build_index(entries: Vec<ExtendedCorpusEntry>) -> Result<Self, BratchError> {
        let mut by_signature = HashMap::with_capacity(RegressionSignature::ALL.len());

        for entry in entries {
            let signature = entry
                .regression_signature
                .parse::<RegressionSignature>()
                .map_err(|_| {
                    BratchError::CorpusLoad(format!(
                        "unrecognised regression_signature in corpus: {}",
                        entry.regression_signature
                    ))
                })?;

            if by_signature.contains_key(&signature) {
                return Err(BratchError::CorpusLoad(format!(
                    "duplicate regression_signature in corpus: {}",
                    entry.regression_signature
                )));
            }

            by_signature.insert(signature, DirectiveString::new(entry.directive));
        }

        for signature in RegressionSignature::ALL {
            if !by_signature.contains_key(&signature) {
                return Err(BratchError::CorpusLoad(format!(
                    "corpus missing entry for regression_signature: {}",
                    signature.stable_name()
                )));
            }
        }

        Ok(Self { by_signature })
    }

    pub fn resolve(&self, signature: RegressionSignature) -> &DirectiveString {
        self.by_signature.get(&signature).expect(
            "RegressionSignatureCorpusIndex invariant: every RegressionSignature variant indexed at construction",
        )
    }
}
