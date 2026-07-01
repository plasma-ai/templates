"""Command-line interface for ``{{ cookiecutter.project.name }}``."""

from __future__ import annotations

from typing import Any

import typer

from . import cmd

__all__ = ['cli']


def cli(**kwargs: Any) -> None:
    """Run the ``{{ cookiecutter.project.name }}`` CLI."""
    # construct app
    kwargs.setdefault('pretty_exceptions_enable', False)
    app = typer.Typer(name='{{ cookiecutter.project.name }}', **kwargs)
    # version callback
    cmd.version(app)
    # {{ cookiecutter.project.name }} commands
    cmd.install(app)
    # run app
    app()


if __name__ == '__main__':
    cli()
