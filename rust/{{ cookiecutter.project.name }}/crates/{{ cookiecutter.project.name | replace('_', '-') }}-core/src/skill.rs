//! The agent skills bundled with the package, and their installation.

use std::fs;
use std::io;
use std::os::unix;
use std::os::unix::fs::PermissionsExt as _;
use std::path::Path;
use std::path::PathBuf;

use include_dir::Dir;
use include_dir::DirEntry;
use include_dir::include_dir;
use tempfile::TempDir;

use crate::path_error::IoResultExt as _;
use crate::path_error::PathError;

/// The skills folder of the repository, embedded at build time.
static SKILLS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../{{ cookiecutter.project.name }}/skills");

/// The skills folder of the source checkout this crate was built from.
///
/// The folder exists only on the machine that built the crate, and only while
/// the checkout does. It is the source [`link`] is given in a development
/// build, and the folder [`install`] never writes into.
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
    /// The new copies are in place, but an earlier one could not be deleted.
    #[error("cannot remove the earlier copies")]
    RemoveEarlier(#[source] PathError),
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
    /// The links are in place, but an earlier copy could not be deleted.
    #[error("cannot remove the earlier copies")]
    RemoveEarlier(#[source] PathError),
}

/// Installs every bundled skill for Claude Code and Codex under `root`.
///
/// Each skill replaces whatever its directory held before. Agent directories
/// that resolve to one place, through a symlink, are written once, and one that
/// resolves to `source`, the skills folder of a source checkout, is left alone:
/// it already serves the skills, and a copy would overwrite their sources. The
/// skills are staged beside their destination and renamed into place, and an
/// earlier copy is moved aside before it is deleted, so a destination never
/// holds a partial copy.
///
/// # Errors
///
/// Returns [`InstallSkillsError::PrepareDirectory`] when a skills directory
/// cannot be created or resolved, [`InstallSkillsError::WriteSkills`] when the
/// staged copy cannot be written, [`InstallSkillsError::ReplaceSkill`] when a
/// copy cannot be moved into place, and [`InstallSkillsError::RemoveEarlier`]
/// when the copy it replaced cannot be deleted.
pub fn install(source: &Path, root: &Path) -> Result<Vec<InstalledSkill>, InstallSkillsError> {
    let targets = prepare_targets(root, source).map_err(InstallSkillsError::PrepareDirectory)?;
    let mut skills: Vec<&Dir<'_>> = SKILLS.dirs().collect();
    skills.sort_by_key(|skill| skill.path());
    let mut installed = Vec::new();
    for target in &targets {
        let stage = Stage::new(target).map_err(InstallSkillsError::WriteSkills)?;
        write_tree(&SKILLS, &stage.new).map_err(InstallSkillsError::WriteSkills)?;
        for skill in &skills {
            let name = skill.path().display().to_string();
            let path = target.join(skill.path());
            if let Err(source) = stage.replace(&name, &path, |path| {
                fs::rename(stage.new.join(&name), path).with_path(path)
            }) {
                return Err(InstallSkillsError::ReplaceSkill { name, source });
            }
            installed.push(InstalledSkill { name, path });
        }
        stage.close().map_err(InstallSkillsError::RemoveEarlier)?;
    }
    Ok(installed)
}

/// Links every skill folder under `source` for Claude Code and Codex under
/// `root`.
///
/// Each link replaces whatever its place held before, so an edit to a skill
/// under `source` applies with no further install. Agent directories that
/// resolve to one place are linked once, and one that resolves to `source`
/// itself is left alone.
///
/// # Errors
///
/// Returns [`LinkSkillsError::ReadSource`] when `source` cannot be read,
/// [`LinkSkillsError::PrepareDirectory`] when a skills directory cannot be
/// created or resolved, [`LinkSkillsError::ReplaceSkill`] when a link cannot be
/// created, and [`LinkSkillsError::RemoveEarlier`] when the copy it replaced
/// cannot be deleted.
pub fn link(source: &Path, root: &Path) -> Result<Vec<InstalledSkill>, LinkSkillsError> {
    let skills = source_skills(source).map_err(LinkSkillsError::ReadSource)?;
    let targets = prepare_targets(root, source).map_err(LinkSkillsError::PrepareDirectory)?;
    let mut linked = Vec::new();
    for target in &targets {
        let stage = Stage::new(target).map_err(LinkSkillsError::PrepareDirectory)?;
        for (name, skill) in &skills {
            let path = target.join(name);
            if let Err(source) = stage.replace(name, &path, |path| {
                unix::fs::symlink(skill, path).with_path(path)
            }) {
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
        stage.close().map_err(LinkSkillsError::RemoveEarlier)?;
    }
    Ok(linked)
}

/// A temporary directory inside an agent's skills directory, on the same
/// filesystem, so entries move in and out of it by rename.
struct Stage {
    dir: TempDir,
    /// Where new copies are written before they move into place.
    new: PathBuf,
    /// Where earlier copies are moved before they are deleted.
    old: PathBuf,
}

impl Stage {
    /// Creates the staging directory inside `target`.
    fn new(target: &Path) -> Result<Self, PathError> {
        let dir = tempfile::Builder::new()
            .prefix(".staged-")
            .tempdir_in(target)
            .with_path(target)?;
        let new = dir.path().join("new");
        let old = dir.path().join("old");
        fs::create_dir(&new).with_path(&new)?;
        fs::create_dir(&old).with_path(&old)?;
        Ok(Self { dir, new, old })
    }

    /// Puts something at `path` with `place`, after moving what `path` held
    /// aside, and moves that back when `place` fails.
    fn replace(
        &self,
        name: &str,
        path: &Path,
        place: impl FnOnce(&Path) -> Result<(), PathError>,
    ) -> Result<(), PathError> {
        let aside = self.old.join(name);
        let moved = match fs::rename(path, &aside) {
            Ok(()) => true,
            Err(error) if error.kind() == io::ErrorKind::NotFound => false,
            Err(error) => return Err(error).with_path(path),
        };
        let placed = place(path);
        if placed.is_err() && moved {
            // The failure of `place` is the one reported; a failed move back
            // leaves the earlier copy in the staging directory's fate.
            let _ = fs::rename(&aside, path);
        }
        placed
    }

    /// Deletes the staging directory, and with it every earlier copy.
    fn close(self) -> Result<(), PathError> {
        let path = self.dir.path().to_path_buf();
        self.dir.close().with_path(&path)
    }
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
/// canonical paths, each once, leaving out one that is `source` itself.
fn prepare_targets(root: &Path, source: &Path) -> Result<Vec<PathBuf>, PathError> {
    // A source that does not exist, as in a released binary, matches nothing.
    let source = fs::canonicalize(source).ok();
    let mut targets = Vec::new();
    for dir in AGENT_DIRS {
        let path = root.join(dir);
        fs::create_dir_all(&path).with_path(&path)?;
        let target = fs::canonicalize(&path).with_path(&path)?;
        if source.as_ref() != Some(&target) && !targets.contains(&target) {
            targets.push(target);
        }
    }
    Ok(targets)
}

/// Writes the embedded entries of `dir` under `root`.
///
/// A file that starts with `#!` is made executable: the embedded tree carries
/// no file modes, and a script is the one kind of file that needs one.
fn write_tree(dir: &Dir<'_>, root: &Path) -> Result<(), PathError> {
    for entry in dir.entries() {
        let path = root.join(entry.path());
        match entry {
            DirEntry::Dir(child) => {
                fs::create_dir(&path).with_path(&path)?;
                write_tree(child, root)?;
            }
            DirEntry::File(file) => {
                fs::write(&path, file.contents()).with_path(&path)?;
                if file.contents().starts_with(b"#!") {
                    let mut permissions = fs::metadata(&path).with_path(&path)?.permissions();
                    // Execute for whoever may read, which keeps to the umask.
                    let mode = permissions.mode();
                    permissions.set_mode(mode | ((mode & 0o444) >> 2));
                    fs::set_permissions(&path, permissions).with_path(&path)?;
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use include_dir::File;

    use super::*;

    #[test]
    fn a_script_is_written_executable_and_other_files_are_not() -> Result<(), PathError> {
        static ENTRIES: [DirEntry<'_>; 2] = [
            DirEntry::File(File::new("run.sh", b"#!/bin/sh\necho hello\n")),
            DirEntry::File(File::new("SKILL.md", b"# A skill\n")),
        ];
        let tree = Dir::new("", &ENTRIES);
        let root = tempfile::tempdir().expect("a temporary directory should be creatable");

        write_tree(&tree, root.path())?;

        let mode =
            |name: &str| fs::metadata(root.path().join(name)).map(|file| file.permissions().mode());
        assert_ne!(mode("run.sh").expect("the script should exist") & 0o111, 0);
        assert_eq!(mode("SKILL.md").expect("the page should exist") & 0o111, 0);
        Ok(())
    }
}
