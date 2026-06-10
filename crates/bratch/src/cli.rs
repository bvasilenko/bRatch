use crate::{BratchError, RegressionSignature, compare};
use clap::{Parser, Subcommand};
use std::{path::PathBuf, process::ExitCode};

#[derive(Debug, Parser)]
#[command(name = "bratch")]
#[command(
    about = "CLI regression-signature detector. Names a diff against a baseline; emits flagged-pattern directive on stdout; exits with a discriminating code so the calling agent can branch."
)]
pub struct BratchCli {
    #[command(subcommand)]
    command: Command,
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

#[derive(Debug, clap::Args)]
pub struct CompareArgs {
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

impl BratchCli {
    pub fn run(self) -> Result<ExitCode, BratchError> {
        match self.command {
            Command::Compare(args) => compare::run(compare::CompareArgs {
                diff: args.diff,
                history: args.history,
                manifest: args.manifest,
                json: args.json,
                quiet: args.quiet,
                reason: args.reason,
            }),
            Command::Signatures => {
                for sig in RegressionSignature::ALL {
                    println!("{sig}");
                }
                Ok(ExitCode::SUCCESS)
            }
            Command::Update => placeholder("update"),
            Command::Init => placeholder("init"),
            Command::Tail => placeholder("tail"),
            Command::Explain => placeholder("explain"),
        }
    }
}

fn placeholder(command_name: &str) -> Result<ExitCode, BratchError> {
    println!("bratch {command_name} placeholder: behavior is deferred.");
    Ok(ExitCode::SUCCESS)
}
