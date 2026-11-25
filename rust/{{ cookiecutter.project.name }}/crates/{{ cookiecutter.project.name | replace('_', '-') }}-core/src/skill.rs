//! The agent skills bundled with the package, and their installation.

use std::fs;
use std::io;
use std::os::unix;
use std::path::Path;
use std::path::PathBuf;

use include_dir::Dir;
use include_dir::DirEntry;
use include_dir::include_dir;

use crate::path_error::IoResultExt as _;
use crate::path_error::PathError;

/// The skills folder of the repository, embedded at build time.
static SKILLS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../{{ cookiecutter.project.name }}/skills");

/// The skills folder of the source checkout this crate was built from.
///
/// The folder exists only on the machine that built the crate, and only while
/// the checkout does: it is the source [`link`] is given in a development
/// build.
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../{{ cookiecutter.project.name }}/skills");

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

/// The failure of [`link`].
#[derive(Debug, thiserror::Error)]
pub enum LinkSkillsError {
    /// The source folder could not be read, as when no checkout holds it.
    #[error("cannot read the skills of the source checkout")]
    ReadSource(#[source] PathError),
    /// An agent's skills directory could not be created or resolved.
    #[error("cannot prepare the skills directory")]
    PrepareDirectory(#[source] PathError),
    /// A link could not take the place of an earlier copy.
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
    let targets = prepare_targets(root).map_err(InstallSkillsError::PrepareDirectory)?;
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

/// Links every skill folder under `source` for Claude Code and Codex under
/// `root`.
///
/// Each link replaces whatever its place held before, so an edit to a skill
/// under `source` applies with no further install. Agent directories that
/// resolve to one place are linked once.
///
/// # Errors
///
/// Returns [`LinkSkillsError::ReadSource`] when `source` cannot be read,
/// [`LinkSkillsError::PrepareDirectory`] when a skills directory cannot be
/// created or resolved, and [`LinkSkillsError::ReplaceSkill`] when an earlier
/// copy cannot be removed or the link created.
pub fn link(source: &Path, root: &Path) -> Result<Vec<InstalledSkill>, LinkSkillsError> {
    let skills = source_skills(source).map_err(LinkSkillsError::ReadSource)?;
    let targets = prepare_targets(root).map_err(LinkSkillsError::PrepareDirectory)?;
    let mut linked = Vec::new();
    for target in &targets {
        for (name, skill) in &skills {
            let path = target.join(name);
            if let Err(source) =
                remove(&path).and_then(|()| unix::fs::symlink(skill, &path).with_path(&path))
            {
                return Err(LinkSkillsError::ReplaceSkill {
                    name: name.clone(),
                    source,
                });
            }
            linked.push(InstalledSkill {
                name: name.clone(),
                path,
            });
        }
    }
    Ok(linked)
}

/// Returns the name and canonical path of each skill folder under `source`,
/// in name order.
fn source_skills(source: &Path) -> Result<Vec<(String, PathBuf)>, PathError> {
    let source = fs::canonicalize(source).with_path(source)?;
    let mut skills = Vec::new();
    for entry in fs::read_dir(&source).with_path(&source)? {
        let entry = entry.with_path(&source)?;
        let path = entry.path();
        if path.is_dir() {
            skills.push((entry.file_name().to_string_lossy().into_owned(), path));
        }
    }
    skills.sort();
    Ok(skills)
}

/// Creates the skills directory of each agent under `root` and returns their
/// canonical paths, each once.
fn prepare_targets(root: &Path) -> Result<Vec<PathBuf>, PathError> {
    let mut targets = Vec::new();
    for dir in AGENT_DIRS {
        let path = root.join(dir);
        fs::create_dir_all(&path).with_path(&path)?;
        let target = fs::canonicalize(&path).with_path(&path)?;
        if !targets.contains(&target) {
            targets.push(target);
        }
    }
    Ok(targets)
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
    remove(path)?;
    fs::rename(staged, path).with_path(path)
}

/// Removes what `path` holds: a directory tree, a file or a link.
fn remove(path: &Path) -> Result<(), PathError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir() => fs::remove_dir_all(path).with_path(path),
        Ok(_) => fs::remove_file(path).with_path(path),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_path(path),
    }
}
