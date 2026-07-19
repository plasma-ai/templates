"""Implements ``{{ cookiecutter.project.name }}`` commands."""

from __future__ import annotations

import importlib.resources
import pathlib
import shutil
from typing import Optional

import typer

import {{ cookiecutter.project.name }}
from {{ cookiecutter.project.name }}.cli.utils import command

__all__ = [
    'version',
    'install',
]


def version(app: typer.Typer) -> typer.Typer:
    """Register the ``--version`` flag on the root callback."""

    def _version_callback(value: bool) -> None:
        """Print the running ``{{ cookiecutter.project.name }}`` package's version and exit."""
        if value:
            typer.echo({{ cookiecutter.project.name }}.__version__)
            raise typer.Exit()

    # version flag
    version_help = 'Show the version and exit.'
    version = typer.Option(
        None,
        '--version',
        callback=_version_callback,
        is_eager=True,
        help=version_help,
    )

    @app.callback()
    def _main(version: Optional[bool] = version) -> None:
        """{{ cookiecutter.project.title }} command-line interface."""

    return app


def install(app: typer.Typer) -> typer.Typer:
    """Register the ``install`` command."""
    # project flag
    project_help = 'Install the skill in cwd rather than home directory.'
    project = typer.Option(False, '--project', help=project_help)
    # link flag
    link_help = (
        'Symlink the bundled skill instead of copying (requires the package'
        ' files on disk, e.g. an editable install), so source edits apply'
        ' without re-installing.'
    )
    link = typer.Option(False, '--link', help=link_help)

    @command(app, 'install')
    def _install(
        project: bool = project,
        link: bool = link,
    ) -> None:
        """Install the {{ cookiecutter.project.name }} skill for Claude Code and Codex.

        Copies the bundled skill into the Claude (.claude/skills) and Codex
        (.agents/skills) skill directories. Targets your home directory by
        default, or the current project with --project. --link symlinks the
        skill instead of copying -- the editable-install dev setup, where
        source edits apply without re-installing.
        """
        # resolve install directory
        if project:
            root = pathlib.Path.cwd()
        else:
            root = pathlib.Path.home()
        # resolve agent skill directories
        targets = [
            root / '.claude' / 'skills',
            root / '.agents' / 'skills',
        ]
        # collect skills
        skills_dir = importlib.resources.files('{{ cookiecutter.project.name }}').joinpath('skills')
        skills = [path for path in skills_dir.iterdir() if path.is_dir()]
        # a symlink needs a real directory to point at; only an on-disk
        # package (an editable install, not a zipped one) provides it
        if link and not isinstance(skills_dir, pathlib.Path):
            raise RuntimeError(
                '--link requires the bundled skill to be a real directory'
                ' (an editable install); a zipped install cannot install'
                ' the skill from the CLI.'
            )
        # copy or link each skill into every target (replaces any prior install)
        for skill in sorted(skills, key=lambda path: path.name):
            for target in targets:
                dest = target / skill.name
                dest.parent.mkdir(parents=True, exist_ok=True)
                if dest.is_symlink() or dest.is_file():
                    dest.unlink()
                elif dest.is_dir():
                    shutil.rmtree(dest)
                if link:
                    dest.symlink_to(skill)
                    typer.echo(f'Linked {skill.name} -> {dest}.')
                else:
                    shutil.copytree(skill, dest)
                    typer.echo(f'Installed {skill.name} -> {dest}.')

    return app
