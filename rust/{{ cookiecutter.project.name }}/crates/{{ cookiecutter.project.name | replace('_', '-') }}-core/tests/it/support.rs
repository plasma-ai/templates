//! Helpers shared by the tests in this binary.

use std::fs;
use std::path::Path;

/// The result a fallible test returns.
pub(crate) type TestResult<T = ()> = anyhow::Result<T>;

/// Writes a fixture file.
pub(crate) fn write_file(path: &Path, contents: &str) -> TestResult {
    fs::write(path, contents)?;
    Ok(())
}

/// Copies the tree under `from` to the new directory `to`.
pub(crate) fn copy_tree(from: &Path, to: &Path) -> TestResult {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}
