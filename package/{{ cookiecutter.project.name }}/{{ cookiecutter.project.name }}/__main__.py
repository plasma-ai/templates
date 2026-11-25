"""Launcher for the ``{{ cookiecutter.project.name }}`` binary."""

from __future__ import annotations

import os
import pathlib
import sys
import sysconfig


def _find_binary() -> pathlib.Path:
    """Return the path of the installed ``{{ cookiecutter.project.name }}`` binary.

    Raises:
        FileNotFoundError: If the binary is in none of the directories an
            installer puts it in.
    """
    name = '{{ cookiecutter.project.name }}' + (sysconfig.get_config_var('EXE') or '')
    package = pathlib.Path(__file__).resolve().parent
    user_scheme = sysconfig.get_preferred_scheme('user')
    directories = [
        # scripts directory of the running interpreter
        pathlib.Path(sysconfig.get_path('scripts')),
        # scripts directory of the base prefix
        pathlib.Path(sysconfig.get_path('scripts', vars={'base': sys.base_prefix})),
        # bin of the prefix the package sits under (a --prefix install):
        # <prefix>/lib/pythonX.Y/site-packages/<package>
        # (the slice is empty when the package sits nearer the root)
        *(prefix / 'bin' for prefix in package.parents[3:4]),
        # bin beside the package (a --target install)
        package.parent / 'bin',
        # scripts directory of the user scheme
        pathlib.Path(sysconfig.get_path('scripts', scheme=user_scheme)),
    ]
    for directory in directories:
        binary = directory / name
        if binary.is_file():
            return binary
    searched = ''.join(f'\n  {directory}' for directory in directories)
    raise FileNotFoundError(f'Binary {name!r} is not installed in any of:{searched}')


def main() -> None:
    """Replace the Python process with the ``{{ cookiecutter.project.name }}`` binary."""
    binary = _find_binary()
    # exec passes signals, the exit status, and the terminal straight
    # through, with no Python process left in the middle
    os.execv(binary, [str(binary), *sys.argv[1:]])


if __name__ == '__main__':
    main()
