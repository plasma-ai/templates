//! The agent skills bundled with the package, and their installation.

use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use include_dir::Dir;
use include_dir::DirEntry;
use include_dir::include_dir;

use crate::path_error::IoResultExt as _;
use crate::path_error::PathError;

/// The skills directory of the Python package, embedded at build time.
static SKILLS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../{{ cookiecutter.project.name }}/skills");

/// The skills directory of each agent, relative to the installation root.
const AGENT_DIRS: [&str; 2] = [".claude/skills", ".agents/skills"];

/// A bundled skill as written into one agent's skills directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledSkill {
    /// The skill's name, which is also the name of its directory.
    pub name: String,
    /// The directory the skill was written to.
    pub path: PathBuf,
}

/// The failure of [`install`].
#[derive(Debug, thiserror::Error)]
pub enum InstallSkillsError {
    /// An agent's skills directory could not be created or resolved.
    #[error("cannot prepare the skills directory")]
    PrepareDirectory(#[source] PathError),
    /// The bundled skills could not be written beside their destination.
    #[error("cannot write the bundled skills")]
    WriteSkills(#[source] PathError),
    /// A new copy could not take the place of an earlier one.
    #[error("cannot replace skill {name:?}")]
    ReplaceSkill {
        /// The name of the skill.
        name: String,
        /// The filesystem call that failed.
        source: PathError,
    },
}

/// Installs every bundled skill for Claude Code and Codex under `root`.
///
/// Each skill replaces whatever its directory held before. Agent directories
/// that resolve to one place, through a symlink, are written once. The skills
/// are staged beside their destination and renamed into place, so a
/// destination never holds a partial copy.
///
/// # Errors
///
/// Returns [`InstallSkillsError::PrepareDirectory`] when a skills directory
/// cannot be created or resolved, [`InstallSkillsError::WriteSkills`] when the
/// staged copy cannot be written, and [`InstallSkillsError::ReplaceSkill`]
/// when an earlier copy cannot be removed or the new one moved into place.
pub fn install(root: &Path) -> Result<Vec<InstalledSkill>, InstallSkillsError> {
    let mut targets = Vec::new();
    for dir in AGENT_DIRS {
        let target = prepare(&root.join(dir)).map_err(InstallSkillsError::PrepareDirectory)?;
        if !targets.contains(&target) {
            targets.push(target);
        }
    }
    let mut skills: Vec<&Dir<'_>> = SKILLS.dirs().collect();
    skills.sort_by_key(|skill| skill.path());
    let mut installed = Vec::new();
    for target in &targets {
        let stage = stage(target).map_err(InstallSkillsError::WriteSkills)?;
        for skill in &skills {
            let name = skill.path().display().to_string();
            let path = target.join(skill.path());
            if let Err(source) = replace(&stage.path().join(skill.path()), &path) {
                return Err(InstallSkillsError::ReplaceSkill { name, source });
            }
            installed.push(InstalledSkill { name, path });
        }
    }
    Ok(installed)
}

/// Creates the skills directory at `path` and returns its canonical path.
fn prepare(path: &Path) -> Result<PathBuf, PathError> {
    fs::create_dir_all(path).with_path(path)?;
    fs::canonicalize(path).with_path(path)
}

/// Writes every bundled skill into a temporary directory inside `target`.
fn stage(target: &Path) -> Result<tempfile::TempDir, PathError> {
    let stage = tempfile::Builder::new()
        .prefix(".staged-")
        .tempdir_in(target)
        .with_path(target)?;
    write_tree(&SKILLS, stage.path())?;
    Ok(stage)
}

/// Writes the embedded entries of `dir` under `root`.
fn write_tree(dir: &Dir<'_>, root: &Path) -> Result<(), PathError> {
    for entry in dir.entries() {
        let path = root.join(entry.path());
        match entry {
            DirEntry::Dir(child) => {
                fs::create_dir(&path).with_path(&path)?;
                write_tree(child, root)?;
            }
            DirEntry::File(file) => fs::write(&path, file.contents()).with_path(&path)?,
        }
    }
    Ok(())
}

/// Moves the staged skill at `staged` to `path`, removing what `path` held.
fn replace(staged: &Path, path: &Path) -> Result<(), PathError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir() => fs::remove_dir_all(path).with_path(path)?,
        Ok(_) => fs::remove_file(path).with_path(path)?,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error).with_path(path),
    }
    fs::rename(staged, path).with_path(path)
}
