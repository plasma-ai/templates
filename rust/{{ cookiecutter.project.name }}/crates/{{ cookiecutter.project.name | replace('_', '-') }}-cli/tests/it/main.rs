//! Integration tests for `{{ cookiecutter.project.name | replace('_', '-') }}-cli`.

use std::fs;
use std::path::Path;

use crate::support::TestResult;

mod completion;
mod help;
mod install;
mod support;

/// Checks that every file in `tests/it/` is declared as a module here.
///
/// Cargo compiles only the modules this file declares, so a test file with no
/// `mod` line would never run.
#[test]
#[cfg_attr(miri, ignore = "Miri isolates tests from the filesystem")]
fn every_test_file_is_declared() -> TestResult {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/it");
    let main = fs::read_to_string(dir.join("main.rs"))?;
    for entry in fs::read_dir(&dir)? {
        let path = entry?.path();
        let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };
        if stem == "main" || path.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }
        let declaration = format!("mod {stem};");
        assert!(
            main.lines().any(|line| line == declaration),
            "tests/it/{stem}.rs is never compiled: add `{declaration}` to tests/it/main.rs",
        );
    }
    Ok(())
}
