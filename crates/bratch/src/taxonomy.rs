use crate::BratchError;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RegressionSignature {
    NullCheckRemoved,
    ErrorHandlingNarrowed,
    TestAssertionWeakened,
    ScopeBoundaryBroken,
    InvariantViolated,
    BrandVoiceLoosened,
    BannedTermReIntroduced,
    DisclosureLineDeleted,
    ApprovedFactReplaced,
}

impl RegressionSignature {
    pub const ALL: [Self; 9] = [
        Self::NullCheckRemoved,
        Self::ErrorHandlingNarrowed,
        Self::TestAssertionWeakened,
        Self::ScopeBoundaryBroken,
        Self::InvariantViolated,
        Self::BrandVoiceLoosened,
        Self::BannedTermReIntroduced,
        Self::DisclosureLineDeleted,
        Self::ApprovedFactReplaced,
    ];

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::NullCheckRemoved => "null-check-removed",
            Self::ErrorHandlingNarrowed => "error-handling-narrowed",
            Self::TestAssertionWeakened => "test-assertion-weakened",
            Self::ScopeBoundaryBroken => "scope-boundary-broken",
            Self::InvariantViolated => "invariant-violated",
            Self::BrandVoiceLoosened => "brand-voice-loosened",
            Self::BannedTermReIntroduced => "banned-term-re-introduced",
            Self::DisclosureLineDeleted => "disclosure-line-deleted",
            Self::ApprovedFactReplaced => "approved-fact-replaced",
        }
    }
}

impl fmt::Display for RegressionSignature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.stable_name())
    }
}

impl FromStr for RegressionSignature {
    type Err = BratchError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "null-check-removed" => Ok(Self::NullCheckRemoved),
            "error-handling-narrowed" => Ok(Self::ErrorHandlingNarrowed),
            "test-assertion-weakened" => Ok(Self::TestAssertionWeakened),
            "scope-boundary-broken" => Ok(Self::ScopeBoundaryBroken),
            "invariant-violated" => Ok(Self::InvariantViolated),
            "brand-voice-loosened" => Ok(Self::BrandVoiceLoosened),
            "banned-term-re-introduced" => Ok(Self::BannedTermReIntroduced),
            "disclosure-line-deleted" => Ok(Self::DisclosureLineDeleted),
            "approved-fact-replaced" => Ok(Self::ApprovedFactReplaced),
            _ => Err(BratchError::TaxonomyUnknown(value.to_owned())),
        }
    }
}
