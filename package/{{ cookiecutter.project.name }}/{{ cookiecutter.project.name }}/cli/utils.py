"""Shared helpers for ``{{ cookiecutter.project.name }}`` CLI commands."""

from __future__ import annotations

import functools
import os
import sys
from collections.abc import Callable
from typing import Any

import typer

__all__ = ['command']


def command(
    app: typer.Typer,
    name: str,
    **kwargs: Any,
) -> Callable:
    """Register a CLI command on ``app`` with error wrapping."""

    def decorator(f: Callable, /) -> Callable:
        if private := name.startswith('_'):
            kwargs.setdefault('hidden', True)

        @functools.wraps(f)
        def wrapper(*args: Any, **kwargs: Any) -> Any:
            try:
                return f(*args, **kwargs)
            except (typer.Exit, typer.Abort, typer.BadParameter):
                raise
            except BrokenPipeError:
                # a downstream reader closed the pipe (not an error):
                # point stdout at devnull so the interpreter's exit
                # flush stays quiet, and end the pipeline successfully
                devnull = os.open(os.devnull, os.O_WRONLY)
                os.dup2(devnull, sys.stdout.fileno())
                raise SystemExit(0) from None
            except Exception as e:
                error = type(e).__name__ if private else 'Error'
                typer.echo(f'{error}: {e}', err=True)
                raise SystemExit(1) from None

        return app.command(name, **kwargs)(wrapper)

    return decorator
