"""Remove the Rust workspace from a project rendered without a Rust toolchain."""

from __future__ import annotations

import pathlib
import shutil

RUST_TOOLCHAIN = '{{ cookiecutter.rust.toolchain }}'
RUST_PATHS = [
    '.config',
    'Cargo.toml',
    'clippy.toml',
    'crates',
    'deny.toml',
    'rust-toolchain.toml',
    'rustfmt.toml',
]

# remove the workspace files when the context pins no toolchain
if not RUST_TOOLCHAIN:
    for name in RUST_PATHS:
        path = pathlib.Path(name)
        if path.is_dir():
            shutil.rmtree(path)
        else:
            path.unlink()
