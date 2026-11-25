//! Tests for the `install` command.

use std::fs;

use insta::assert_snapshot;

use crate::support::Sandbox;
use crate::support::TestResult;

/// Checks that the skill lands in both agent directories of the home
/// directory, and that the narration is on stderr.
#[test]
fn install_writes_the_skill_for_both_agents() -> TestResult {
    let sandbox = Sandbox::new()?;
    assert_snapshot!(sandbox.run(&["install"])?, @"
    ------- stderr -------
    Installed {{ cookiecutter.project.name }} -> [ROOT]/home/.claude/skills/{{ cookiecutter.project.name }}.
    Installed {{ cookiecutter.project.name }} -> [ROOT]/home/.agents/skills/{{ cookiecutter.project.name }}.
    ");
    for agent in [".claude", ".agents"] {
        let skill = sandbox.home().join(agent).join("skills/{{ cookiecutter.project.name }}/SKILL.md");
        assert!(skill.is_file(), "{} should exist", skill.display());
    }
    Ok(())
}

/// Checks that `--project` installs into the working directory, and that
/// `--quiet` silences the narration.
#[test]
fn quiet_project_install_writes_into_the_working_directory_in_silence() -> TestResult {
    let sandbox = Sandbox::new()?;
    assert_snapshot!(sandbox.run(&["--quiet", "install", "--project"])?, @"");
    let skill = sandbox.work().join(".claude/skills/{{ cookiecutter.project.name }}/SKILL.md");
    assert!(skill.is_file(), "{} should exist", skill.display());
    Ok(())
}

/// Checks that a failure prints its whole cause chain and exits 2.
#[test]
fn a_failure_prints_its_cause_chain() -> TestResult {
    let sandbox = Sandbox::new()?;
    fs::write(
        sandbox.home().join(".claude"),
        "a file where a directory belongs",
    )?;
    assert_snapshot!(sandbox.run(&["install"])?, @r#"
    ------- stderr -------
    error: cannot prepare the skills directory
    caused by: cannot access "[ROOT]/home/.claude/skills"
    caused by: Not a directory (os error 20)
    [exit status: 2]
    "#);
    Ok(())
}

/// Checks that a failure the CLI knows a remedy for ends with its hint.
#[test]
fn a_missing_home_directory_fails_with_a_hint() -> TestResult {
    let sandbox = Sandbox::new()?;
    let mut command = sandbox.command(&["install"]);
    command.env_remove("HOME");
    assert_snapshot!(sandbox.observe(command)?, @"
    ------- stderr -------
    error: cannot find the home directory, because `HOME` is not set
    hint: pass `--project` to install into the current directory
    [exit status: 2]
    ");
    Ok(())
}
