//! Command-line interface for `{{ cookiecutter.project.name }}`.
//!
//! The crate exports one function, [`main`], which the `{{ cookiecutter.project.name }}` binary
//! calls.

use std::io;
use std::io::Write as _;
use std::panic;
use std::process::ExitCode;

use clap::Parser as _;

use crate::cli::Cli;
use crate::command::CommandError;
use crate::command::CommandOutcome;
use crate::config::Config;
use crate::interrupt::Interrupt;
use crate::output::Output;

mod cli;
mod command;
mod config;
mod interrupt;
mod output;

/// Where a bug in the program is reported.
const ISSUES_URL: &str = "https://github.com/{{ cookiecutter.github.org or cookiecutter.github.user }}/{{ cookiecutter.github.repo }}/issues";

/// Runs the command line and returns the status the process exits with.
///
/// A positive outcome exits 0 and a negative one 1. A failure is printed and
/// exits 2, as a usage error does; a reader that closed stdout early is not a
/// failure and exits 0. A run that received SIGINT or SIGTERM ends by that
/// signal. A panic exits 101.
pub fn main() -> ExitCode {
    install_panic_hook();
    let cli = Cli::parse();
    let config = Config::from_env();
    let interrupt = Interrupt::new();
    let mut output = Output::new(cli.global.color, cli.global.quiet);
    let result = cli
        .command
        .run(&mut output, &config, &interrupt)
        .and_then(|outcome| {
            output.flush()?;
            Ok(outcome)
        });
    if let Some(signal) = interrupt.signal() {
        return interrupt::terminate(signal);
    }
    match result {
        Ok(CommandOutcome::Success) => ExitCode::SUCCESS,
        Ok(CommandOutcome::Negative) => ExitCode::from(1),
        Err(CommandError::Stdout(error)) if error.is_closed_pipe() => ExitCode::SUCCESS,
        Err(error) => {
            output.error(&error);
            ExitCode::from(2)
        }
    }
}

/// Adds a line to the default panic message that says where to report it.
#[expect(
    clippy::disallowed_methods,
    reason = "a panic can unwind through the output type, so the hook writes to stderr itself"
)]
fn install_panic_hook() {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        default_hook(info);
        // The process is already failing, and stderr is the only place left to
        // report a failed write to.
        let _ = writeln!(
            io::stderr(),
            "This is a bug in {{ cookiecutter.project.name }} {}. Please report it at {ISSUES_URL}.",
            env!("CARGO_PKG_VERSION"),
        );
    }));
}
