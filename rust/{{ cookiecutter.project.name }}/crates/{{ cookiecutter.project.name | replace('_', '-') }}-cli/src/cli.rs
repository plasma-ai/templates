//! The argument types of the command line.

use clap::Args;
use clap::ColorChoice;
use clap::Parser;
use clap::error::ErrorKind;

use crate::command::Command;

/// {{ cookiecutter.project.title }} command-line interface
#[derive(Debug, Parser)]
#[command(name = "{{ cookiecutter.project.name }}", version)]
pub(crate) struct Cli {
    #[command(flatten)]
    pub(crate) global: GlobalArgs,
    #[command(subcommand)]
    pub(crate) command: Command,
}

impl Cli {
    /// Parses the command line, or returns `None` when it asks for the
    /// version.
    ///
    /// clap prints help and usage errors itself and ends the process. It
    /// would print the version as `<tool> <version>`, where every org tool
    /// prints the bare version, so that request goes back to the caller.
    pub(crate) fn parse_or_version() -> Option<Self> {
        match Self::try_parse() {
            Ok(cli) => Some(cli),
            Err(error) if error.kind() == ErrorKind::DisplayVersion => None,
            Err(error) => error.exit(),
        }
    }
}

// A doc comment here would replace the help of the command it is flattened
// into.
#[derive(Debug, Args)]
#[command(next_help_heading = "Global Options")]
pub(crate) struct GlobalArgs {
    /// Control when output is colored
    #[arg(long, global = true, value_name = "WHEN", default_value_t = ColorChoice::Auto)]
    pub(crate) color: ColorChoice,
    /// Print no status messages or hints
    #[arg(short, long, global = true)]
    pub(crate) quiet: bool,
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory as _;

    use super::*;

    #[test]
    fn argument_definitions_are_consistent() {
        Cli::command().debug_assert();
    }

    #[test]
    fn every_command_parses_with_only_its_required_arguments() -> Result<(), clap::Error> {
        for arguments in [&["install"][..], &["completion", "bash"]] {
            Cli::try_parse_from(["{{ cookiecutter.project.name }}"].iter().chain(arguments))?;
        }
        Ok(())
    }
}
