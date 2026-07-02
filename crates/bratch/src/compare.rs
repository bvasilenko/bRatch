use crate::{BratchError, RegressionSignature, corpus_index::RegressionSignatureCorpusIndex};
use bsuite_core::{ExitCode, prompt_resolver::DirectiveString};

pub fn run(
    args: &crate::cli::CompareArgs,
    corpus: &RegressionSignatureCorpusIndex,
) -> Result<(DirectiveString, ExitCode), BratchError> {
    crate::substrate_input::SubstrateInput::new(args.diff.clone(), args.history.clone())?;
    validate_reason(args.reason.as_deref())?;
    let signature = args.signature.parse::<RegressionSignature>()?;
    let directive = corpus.resolve(signature).clone();
    Ok((directive, ExitCode::Finding))
}

fn validate_reason(reason: Option<&str>) -> Result<(), BratchError> {
    if matches!(reason, Some(reason) if reason.trim().is_empty()) {
        return Err(BratchError::Usage("reason must not be empty".to_owned()));
    }
    Ok(())
}
