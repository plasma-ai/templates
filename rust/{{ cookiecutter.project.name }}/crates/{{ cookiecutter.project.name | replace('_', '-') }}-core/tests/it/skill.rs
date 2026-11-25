//! Tests for the installation of the bundled skills.

use std::collections::BTreeMap;
use std::fs;
use std::os::unix;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use {{ cookiecutter.project.name }}_core::skill;
use {{ cookiecutter.project.name }}_core::skill::LinkSkillsError;
use tempfile::TempDir;

use crate::support;
use crate::support::TestResult;

/// The prefix of the temporary directory each test creates.
const ROOT_PREFIX: &str = "{{ cookiecutter.project.name }}-test-";

/// Checks that an install copies the package's skills for both agents, and
/// that a second install puts a changed copy right.
#[test]
fn install_copies_the_bundled_skills_and_replaces_an_earlier_copy() -> TestResult {
    let (_guard, root) = temp_root()?;
    let bundled = read_tree(&bundled_skills())?;

    let installed = skill::install(&bundled_skills(), &root)?;
    assert_eq!(read_tree(&root.join(".claude/skills"))?, bundled);
    assert_eq!(read_tree(&root.join(".agents/skills"))?, bundled);

    let first = installed.first().context("a skill should be bundled")?;
    support::write_file(&first.path.join("stray.md"), "left by an earlier version")?;
    fs::remove_file(first.path.join("SKILL.md"))?;
    assert_eq!(skill::install(&bundled_skills(), &root)?, installed);
    assert_eq!(read_tree(&root.join(".claude/skills"))?, bundled);
    assert_eq!(read_tree(&root.join(".agents/skills"))?, bundled);
    Ok(())
}

/// Checks that agent directories linked to one place receive one copy.
#[test]
fn install_writes_linked_agent_directories_once() -> TestResult {
    let (_guard, root) = temp_root()?;
    let claude = root.join(".claude/skills");
    fs::create_dir_all(&claude)?;
    fs::create_dir(root.join(".agents"))?;
    unix::fs::symlink(&claude, root.join(".agents/skills"))?;

    let installed = skill::install(&bundled_skills(), &root)?;
    let bundled = fs::read_dir(bundled_skills())?.count();
    assert_eq!(installed.len(), bundled);
    for skill in &installed {
        assert_eq!(skill.path.parent(), Some(claude.as_path()));
    }
    Ok(())
}

/// Checks that a link replaces an installed copy and points each agent at the
/// source folder, so an edit there needs no further install.
#[test]
fn link_points_both_agents_at_the_source_folder() -> TestResult {
    let (_guard, root) = temp_root()?;
    let source = bundled_skills().canonicalize()?;
    let installed = skill::install(&bundled_skills(), &root)?;

    let linked = skill::link(&source, &root)?;
    assert_eq!(linked, installed);
    for skill in &linked {
        assert_eq!(fs::read_link(&skill.path)?, source.join(&skill.name));
    }
    assert_eq!(
        read_tree(&root.join(".agents/skills"))?,
        read_tree(&source)?
    );
    Ok(())
}

/// Checks that a source folder that is not there is a typed failure, which is
/// what a binary built on another machine meets.
#[test]
fn link_fails_when_the_source_folder_is_missing() -> TestResult {
    let (_guard, root) = temp_root()?;
    let result = skill::link(&root.join("no-checkout"), &root);
    assert!(
        matches!(result, Err(LinkSkillsError::ReadSource(_))),
        "{result:?}"
    );
    Ok(())
}

/// Checks that an agent directory which is the source folder itself is left
/// alone, so neither a copy nor a link can overwrite the skills' sources.
#[test]
fn an_agent_directory_that_is_the_source_folder_is_left_alone() -> TestResult {
    let (_guard, root) = temp_root()?;
    let bundled = read_tree(&bundled_skills())?;
    let source = root.join("checkout/skills");
    support::copy_tree(&bundled_skills(), &source)?;
    fs::create_dir(root.join(".claude"))?;
    unix::fs::symlink(&source, root.join(".claude/skills"))?;

    let installed = skill::install(&source, &root)?;
    assert_eq!(read_tree(&source)?, bundled);
    let linked = skill::link(&source, &root)?;
    assert_eq!(read_tree(&source)?, bundled);
    for skill in installed.iter().chain(&linked) {
        assert!(
            skill.path.starts_with(root.join(".agents/skills")),
            "{skill:?}"
        );
    }
    Ok(())
}

/// Returns the skills folder of the repository in the source tree.
fn bundled_skills() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../{{ cookiecutter.project.name }}/skills")
}

/// Creates a temporary directory for one test and returns it with its
/// canonical path, since a temporary directory is often behind a symlink.
fn temp_root() -> TestResult<(TempDir, PathBuf)> {
    let guard = tempfile::Builder::new().prefix(ROOT_PREFIX).tempdir()?;
    let root = guard.path().canonicalize()?;
    Ok((guard, root))
}

/// Reads the tree under `root` as relative paths, each with its contents when
/// it is a file.
fn read_tree(root: &Path) -> TestResult<BTreeMap<PathBuf, Option<Vec<u8>>>> {
    let mut tree = BTreeMap::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(dir) = pending.pop() {
        for entry in fs::read_dir(&dir)? {
            let path = entry?.path();
            let relative = path.strip_prefix(root)?.to_path_buf();
            if path.is_dir() {
                tree.insert(relative, None);
                pending.push(path);
            } else {
                tree.insert(relative, Some(fs::read(&path)?));
            }
        }
    }
    Ok(tree)
}
