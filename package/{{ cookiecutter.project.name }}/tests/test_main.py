"""Test the ``{{ cookiecutter.project.name }}.__main__`` module."""

from __future__ import annotations

import subprocess
import sys

import {{ cookiecutter.project.name }}

__all__ = ['test_main_runs_installed_binary']


def test_main_runs_installed_binary() -> None:
    """Test that ``python -m {{ cookiecutter.project.name }}`` runs the installed binary."""
    result = subprocess.run(
        [sys.executable, '-m', '{{ cookiecutter.project.name }}', '--version'],
        capture_output=True,
        text=True,
        check=True,
    )
    version = {{ cookiecutter.project.name }}.__version__
    assert result.stdout == f'{version}\n'
