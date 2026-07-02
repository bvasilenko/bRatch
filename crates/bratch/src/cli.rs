use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "bratch")]
#[command(
    about = "CLI regression-signature detector. Names a diff against a baseline; emits flagged-pattern directive on stdout; exits with a discriminating code so the calling agent can branch."
)]
pub struct BratchCli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Compare(CompareArgs),
    Signatures,
    Update,
    Init,
    Tail,
    Explain,
}

#[derive(Debug, Clone, Eq, PartialEq, clap::Args)]
pub struct CompareArgs {
    #[arg(long, value_name = "regression-signature")]
    pub signature: String,
    #[arg(long, value_name = "path")]
    pub diff: Option<PathBuf>,
    #[arg(long, value_name = "path-or-name")]
    pub history: Option<String>,
    #[arg(long, value_name = "path")]
    pub manifest: Option<PathBuf>,
    #[arg(long)]
    pub json: bool,
    #[arg(long)]
    pub quiet: bool,
    #[arg(long, value_name = "text")]
    pub reason: Option<String>,
}
