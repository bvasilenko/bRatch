use crate::BratchError;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VerdictClass {
    NoRegression,
    RegressionDetected,
    Malformed,
}

impl VerdictClass {
    pub const ALL: [Self; 3] = [
        Self::NoRegression,
        Self::RegressionDetected,
        Self::Malformed,
    ];

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::NoRegression => "no-regression",
            Self::RegressionDetected => "regression-detected",
            Self::Malformed => "malformed",
        }
    }
}

impl fmt::Display for VerdictClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.stable_name())
    }
}

impl FromStr for VerdictClass {
    type Err = BratchError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "no-regression" => Ok(Self::NoRegression),
            "regression-detected" => Ok(Self::RegressionDetected),
            "malformed" => Ok(Self::Malformed),
            _ => Err(BratchError::VerdictClassUnknown(value.to_owned())),
        }
    }
}

impl From<VerdictClass> for bsuite_core::ExitCode {
    fn from(value: VerdictClass) -> Self {
        match value {
            VerdictClass::NoRegression => Self::Success,
            VerdictClass::RegressionDetected => Self::Finding,
            VerdictClass::Malformed => Self::InternalError,
        }
    }
}
