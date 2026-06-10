use thiserror::Error;

#[derive(Debug, Error)]
pub enum BratchError {
    #[error("diff path is invalid: {0}")]
    DiffSliceInvalid(String),
    #[error("history ref is invalid: {0}")]
    HistoryRefInvalid(String),
    #[error("unknown regression signature: {0}")]
    TaxonomyUnknown(String),
    #[error("unknown verdict class: {0}")]
    VerdictClassUnknown(String),
    #[error("malformed revision id pair: {0}")]
    RevisionIdPairMalformed(String),
    #[error("unknown invocation surface: {0}")]
    InvocationSurfaceUnknown(String),
    #[error("{0}")]
    Usage(String),
    #[error(transparent)]
    Core(#[from] bsuite_core::BsuiteCoreError),
}

impl BratchError {
    pub const fn exit_code(&self) -> bsuite_core::ExitCode {
        match self {
            Self::Usage(_) => bsuite_core::ExitCode::Usage,
            Self::DiffSliceInvalid(_)
            | Self::HistoryRefInvalid(_)
            | Self::TaxonomyUnknown(_)
            | Self::VerdictClassUnknown(_)
            | Self::RevisionIdPairMalformed(_)
            | Self::InvocationSurfaceUnknown(_)
            | Self::Core(_) => bsuite_core::ExitCode::InternalError,
        }
    }

    pub fn process_exit_code(&self) -> std::process::ExitCode {
        process_exit_code(self.exit_code())
    }
}

pub fn process_exit_code(code: bsuite_core::ExitCode) -> std::process::ExitCode {
    std::process::ExitCode::from(code.as_i32() as u8)
}
