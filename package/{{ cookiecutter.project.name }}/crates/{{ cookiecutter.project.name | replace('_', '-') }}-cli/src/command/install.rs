//! The `install` command.

use std::path::Path;

use clap::Args;
use {{ cookiecutter.project.name }}_core::skill;

use crate::command::CommandError;
use crate::command::CommandOutcome;
use crate::config::Config;
use crate::interrupt::Interrupt;
use crate::output::Output;

/// Install the {{ cookiecutter.project.name }} skill for Claude Code and Codex
///
/// Copies the bundled skill into the Claude (`.claude/skills`) and Codex
/// (`.agents/skills`) skill directories, replacing any earlier copy. The
/// directories are those of your home directory, or of the current directory
/// with `--project`.
#[derive(Debug, Args)]
pub(crate) struct InstallArgs {
    /// Install into the current directory instead of the home directory
    #[arg(long)]
    project: bool,
}

/// Runs the `install` command.
pub(crate) fn run(
    output: &mut Output,
    args: &InstallArgs,
    config: &Config,
    interrupt: &Interrupt,
) -> Result<CommandOutcome, CommandError> {
    // The copy is staged in a temporary directory, which only an unwinding
    // stack removes.
    interrupt.watch().map_err(CommandError::WatchSignals)?;
    let root = if args.project {
        Path::new(".")
    } else {
        config
            .home
            .as_deref()
            .ok_or(CommandError::NoHomeDirectory)?
    };
    for skill in skill::install(root)? {
        output.status(&format!(
            "Installed {} -> {}.",
            skill.name,
            skill.path.display()
        ))?;
    }
    Ok(CommandOutcome::Success)
}
