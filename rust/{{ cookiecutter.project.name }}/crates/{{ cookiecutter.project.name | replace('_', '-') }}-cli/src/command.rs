//! The commands, their dispatch, and the result every command returns.

use std::io;

use clap::Subcommand;
use {{ cookiecutter.project.name }}_core::skill::InstallSkillsError;
use {{ cookiecutter.project.name }}_core::skill::LinkSkillsError;

use crate::config::Config;
use crate::interrupt::Interrupt;
use crate::output::Output;
use crate::output::StdoutError;

mod completion;
mod install;

// A variant carries no doc comment: clap would take it over the help written
// on the command's `Args` type.
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    Install(install::InstallArgs),
    Completion(completion::CompletionArgs),
}

impl Command {
    /// Runs the command and returns its outcome.
    pub(crate) fn run(
        &self,
        output: &mut Output,
        config: &Config,
        interrupt: &Interrupt,
    ) -> Result<CommandOutcome, CommandError> {
        match self {
            Self::Install(args) => install::run(output, args, config, interrupt),
            Self::Completion(args) => completion::run(output, args),
        }
    }
}

/// A command's own answer, which the exit status reports.
#[derive(Debug)]
pub(crate) enum CommandOutcome {
    /// The command did what was asked, or its answer is yes.
    Success,
    /// The command ran and its answer is no, as when a check finds a problem.
    #[expect(dead_code, reason = "no command has a negative answer yet")]
    Negative,
}

/// Every failure the binary reports, sorted for the exit mapping in `main`.
#[derive(Debug, thiserror::Error)]
pub(crate) enum CommandError {
    /// A write to stdout failed.
    #[error(transparent)]
    Stdout(#[from] StdoutError),
    /// The bundled skills could not be installed.
    #[error(transparent)]
    InstallSkills(#[from] InstallSkillsError),
    /// The skills of the source checkout could not be linked.
    #[error(transparent)]
    LinkSkills(#[from] LinkSkillsError),
    /// The environment names no home directory.
    #[error("cannot find the home directory, because `HOME` is not set")]
    NoHomeDirectory,
    /// The handlers for SIGINT and SIGTERM could not be installed.
    #[error("cannot watch for an interrupt")]
    WatchSignals(#[source] io::Error),
}

impl CommandError {
    /// Returns the remedies the CLI knows for this failure.
    pub(crate) fn hints(&self) -> &'static [&'static str] {
        match self {
            Self::NoHomeDirectory => &["pass `--project` to install into the current directory"],
            Self::LinkSkills(LinkSkillsError::ReadSource(_)) => &[
                "`--link` needs a binary built in a development clone",
                "run `install` without `--link` to copy the skill",
            ],
            Self::Stdout(_)
            | Self::InstallSkills(_)
            | Self::LinkSkills(_)
            | Self::WatchSignals(_) => &[],
        }
    }
}
