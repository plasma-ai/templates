//! Tests for the help and version output, which pin the CLI surface.

use insta::assert_snapshot;

use crate::support::Sandbox;
use crate::support::TestResult;

/// Checks the long help of the root and of every command.
#[test]
fn help_describes_every_command_and_option() -> TestResult {
    let sandbox = Sandbox::new()?;
    assert_snapshot!(sandbox.run(&["--help"])?, @"
    {{ cookiecutter.project.title }} command-line interface

    Usage: {{ cookiecutter.project.name }} [OPTIONS] <COMMAND>

    Commands:
      install     Install the {{ cookiecutter.project.name }} skill for Claude Code and Codex
      completion  Print a shell completion script
      help        Print this message or the help of the given subcommand(s)

    Options:
      -h, --help     Print help
      -V, --version  Print version

    Global Options:
          --color <WHEN>  Control when output is colored [default: auto] [possible values: auto, always, never]
      -q, --quiet         Print no status messages or hints
    ");
    assert_snapshot!(sandbox.run(&["install", "--help"])?, @"
    Install the {{ cookiecutter.project.name }} skill for Claude Code and Codex

    Copies the bundled skill into the Claude (`.claude/skills`) and Codex (`.agents/skills`) skill directories, replacing any earlier copy. The directories are those of your home directory, or of the current directory with `--project`.

    Usage: {{ cookiecutter.project.name }} install [OPTIONS]

    Options:
          --project
              Install into the current directory instead of the home directory

      -h, --help
              Print help (see a summary with '-h')

    Global Options:
          --color <WHEN>
              Control when output is colored

              [default: auto]
              [possible values: auto, always, never]

      -q, --quiet
              Print no status messages or hints
    ");
    assert_snapshot!(sandbox.run(&["completion", "--help"])?, @"
    Print a shell completion script

    Writes the script for the given shell to stdout. The shell's own manual says where a completion script is installed.

    Usage: {{ cookiecutter.project.name }} completion [OPTIONS] <SHELL>

    Arguments:
      <SHELL>
              Shell to write the script for

              [possible values: bash, elvish, fish, powershell, zsh]

    Options:
      -h, --help
              Print help (see a summary with '-h')

    Global Options:
          --color <WHEN>
              Control when output is colored

              [default: auto]
              [possible values: auto, always, never]

      -q, --quiet
              Print no status messages or hints
    ");
    Ok(())
}

/// Checks that `--version` prints the bare workspace version, with no name
/// before it, so a script can parse the output as a version.
#[test]
fn version_prints_the_bare_workspace_version() -> TestResult {
    let sandbox = Sandbox::new()?;
    let output = sandbox.run(&["--version"])?;
    assert_eq!(output.to_string(), concat!(env!("CARGO_PKG_VERSION"), "\n"));
    Ok(())
}
