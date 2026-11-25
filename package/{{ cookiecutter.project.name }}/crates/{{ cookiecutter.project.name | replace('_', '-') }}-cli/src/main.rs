//! The `{{ cookiecutter.project.name }}` command.

use std::process::ExitCode;

fn main() -> ExitCode {
    {{ cookiecutter.project.name }}_cli::main()
}
