use crate::{
    BratchError, RegressionSignature, VerdictClass, error::process_exit_code,
    substrate_input::SubstrateInput,
};
use std::{fmt, path::Path};

const BINARY_NAME: &str = "bratch";
const INVOCATION_SURFACE: &str = "cli";
const PLACEHOLDER_SIGNATURE: RegressionSignature = RegressionSignature::TestAssertionWeakened;
const PLACEHOLDER_VERDICT: VerdictClass = VerdictClass::RegressionDetected;

pub struct CompareArgs {
    pub diff: Option<std::path::PathBuf>,
    pub history: Option<String>,
    pub manifest: Option<std::path::PathBuf>,
    pub json: bool,
    pub quiet: bool,
    pub reason: Option<String>,
}

pub fn run(args: CompareArgs) -> Result<std::process::ExitCode, BratchError> {
    validate_reason(args.reason.as_deref())?;

    let input = SubstrateInput::new(args.diff, args.history)?;

    println!("{}", PlaceholderDirective::new(input));

    Ok(process_exit_code(PLACEHOLDER_VERDICT.into()))
}

fn validate_reason(reason: Option<&str>) -> Result<(), BratchError> {
    if matches!(reason, Some(reason) if reason.trim().is_empty()) {
        return Err(BratchError::Usage("reason must not be empty".to_owned()));
    }

    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PlaceholderDirective {
    input: SubstrateInput,
    signature: RegressionSignature,
    verdict: VerdictClass,
    invocation_surface: &'static str,
}

impl PlaceholderDirective {
    fn new(input: SubstrateInput) -> Self {
        Self {
            input,
            signature: PLACEHOLDER_SIGNATURE,
            verdict: PLACEHOLDER_VERDICT,
            invocation_surface: INVOCATION_SURFACE,
        }
    }
}

impl fmt::Display for PlaceholderDirective {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let diff = format_optional_path(self.input.diff.as_deref());
        let history = format_optional_str(self.input.history.as_deref());
        let signature_variant = format_variant_name(self.signature);
        let verdict_name = self.verdict.stable_name();

        writeln!(
            formatter,
            "[{BINARY_NAME} placeholder directive - pre-corpus output]"
        )?;
        writeln!(
            formatter,
            "Parsed input: diff={diff}, history={history}. \
             Routing key: RegressionSignature::{signature_variant} placeholder route. \
             Invocation surface: {}.",
            self.invocation_surface,
        )?;
        writeln!(formatter, "Verdict-state: {verdict_name}.")?;
        writeln!(
            formatter,
            "ACTION: This invocation reached {BINARY_NAME} at the pre-corpus phase. \
             A real evolved directive would name the specific regression signature \
             {signature_variant} matched in the diff against the baseline and steer \
             the calling LLM toward either reverting the offending change or extending \
             the test surface so the regression is locked out."
        )?;
        write!(
            formatter,
            "Re-invoke after the corpus-backed release lands. \
             Do not treat this placeholder as ground truth. \
             Exit code carries the verdict-class signal."
        )
    }
}

fn format_optional_path(path: Option<&Path>) -> String {
    path.and_then(|p| p.to_str())
        .map(str::to_owned)
        .unwrap_or_else(|| "<none>".to_owned())
}

fn format_optional_str(value: Option<&str>) -> String {
    value
        .map(str::to_owned)
        .unwrap_or_else(|| "<none>".to_owned())
}

fn format_variant_name<T: fmt::Debug>(value: T) -> String {
    format!("{value:?}")
}
