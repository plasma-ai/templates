"""Remove the files a project's Rust settings do not call for."""

from __future__ import annotations

import pathlib
import shutil
import subprocess

RUST_TOOLCHAIN = '{{ cookiecutter.rust_toolchain }}'
RUST_BINDINGS = '{{ cookiecutter.rust_bindings }}'
PACKAGE = '{{ cookiecutter.project.name }}'
CRATE = "{{ cookiecutter.project.name | replace('_', '-') }}"

# the Cargo workspace
RUST_PATHS = [
    '.config',
    'Cargo.toml',
    'clippy.toml',
    'crates',
    'deny.toml',
    'rust-toolchain.toml',
    'rustfmt.toml',
]
# the Rust CLI, its launcher, and the skill installer behind it
BINARY_PATHS = [
    f'{PACKAGE}/__main__.py',
    f'crates/{CRATE}-cli',
    f'crates/{CRATE}-core/build.rs',
    f'crates/{CRATE}-core/src/path_error.rs',
    f'crates/{CRATE}-core/src/skill.rs',
    f'crates/{CRATE}-core/tests/it/skill.rs',
    'tests/test_main.py',
]
# the Python CLI and package skeleton, which the Rust CLI replaces
PYTHON_PATHS = [
    f'{PACKAGE}/cli',
    f'{PACKAGE}/constants.py',
    f'{PACKAGE}/core',
    f'{PACKAGE}/exceptions.py',
    f'{PACKAGE}/typing.py',
    f'{PACKAGE}/util',
]
# the import layout the cargo-fmt pre-commit hook passes to rustfmt
RUSTFMT_CONFIG = 'group_imports=StdExternalCrate,imports_granularity=Item'


def remove(names: list[str]) -> None:
    """Remove each named file or directory from the rendered project."""
    for name in names:
        path = pathlib.Path(name)
        if path.is_dir():
            shutil.rmtree(path)
        else:
            path.unlink()


# keep one command-line interface: the Rust one in a binary wheel
if RUST_BINDINGS == 'bin':
    remove(PYTHON_PATHS)
    # rustfmt orders imports and breaks lines by the rendered project name,
    # so the sources are formatted here, as the pre-commit hook would
    subprocess.run(
        ['cargo', 'fmt', '--all', '--', '--config', RUSTFMT_CONFIG],
        check=True,
    )
else:
    remove(BINARY_PATHS)
# remove the workspace files when the context pins no toolchain
if not RUST_TOOLCHAIN:
    remove(RUST_PATHS)
