use crate::BratchError;
use std::{fmt, path::PathBuf, str::FromStr};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RevisionId(pub String);

impl fmt::Display for RevisionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RevisionIdPair {
    pub baseline: RevisionId,
    pub candidate: RevisionId,
}

impl fmt::Display for RevisionIdPair {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}..{}", self.baseline, self.candidate)
    }
}

impl FromStr for RevisionIdPair {
    type Err = BratchError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let malformed = || BratchError::RevisionIdPairMalformed(value.to_owned());

        if value.contains("...") {
            return Err(malformed());
        }

        let (baseline_raw, candidate_raw) = value.split_once("..").ok_or_else(malformed)?;

        if baseline_raw.is_empty() || candidate_raw.is_empty() {
            return Err(malformed());
        }

        if candidate_raw.contains("..") {
            return Err(malformed());
        }

        Ok(Self {
            baseline: RevisionId(baseline_raw.to_owned()),
            candidate: RevisionId(candidate_raw.to_owned()),
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SubstrateInput {
    pub diff: Option<PathBuf>,
    pub history: Option<String>,
}

impl SubstrateInput {
    pub fn new(diff: Option<PathBuf>, history: Option<String>) -> Result<Self, BratchError> {
        if matches!(diff.as_ref().and_then(|p| p.to_str()), Some("")) {
            return Err(BratchError::DiffSliceInvalid(
                "diff path must not be empty".to_owned(),
            ));
        }

        if matches!(history.as_deref(), Some("")) {
            return Err(BratchError::HistoryRefInvalid(
                "history ref must not be empty".to_owned(),
            ));
        }

        Ok(Self { diff, history })
    }
}
