//! The path context that every filesystem error carries.

use std::io;
use std::path::Path;
use std::path::PathBuf;

/// A failed filesystem call, with the path it was given.
#[derive(Debug, thiserror::Error)]
#[error("cannot access {path:?}")]
pub struct PathError {
    /// The path the call was given.
    pub path: PathBuf,
    /// The error the call returned.
    pub source: io::Error,
}

/// Adds path context to the result of a filesystem call.
pub trait IoResultExt<T> {
    /// Wraps a failure in a [`PathError`] that names `path`.
    ///
    /// # Errors
    ///
    /// Returns the [`PathError`] when `self` is an error.
    fn with_path(self, path: &Path) -> Result<T, PathError>;
}

impl<T> IoResultExt<T> for io::Result<T> {
    fn with_path(self, path: &Path) -> Result<T, PathError> {
        self.map_err(|source| PathError {
            path: path.to_path_buf(),
            source,
        })
    }
}
