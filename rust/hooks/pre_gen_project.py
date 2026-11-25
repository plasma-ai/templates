"""Reject a context the Rust template cannot render."""

from __future__ import annotations

import re
import shutil
import sys

PROJECT_NAME = '{{ cookiecutter.project.name }}'
RUST_TOOLCHAIN = '{{ cookiecutter.rust_toolchain }}'

# the tool name becomes Rust identifiers and, with dashes, crate names
if re.fullmatch(r'[a-z][a-z0-9_]*', PROJECT_NAME) is None:
    sys.exit(
        'Error: project.name must start with a lowercase letter and hold only'
        f' lowercase letters, digits and underscores (got {PROJECT_NAME!r})'
    )
# the toolchain pin is one exact stable release
if re.fullmatch(r'\d+\.\d+\.\d+', RUST_TOOLCHAIN) is None:
    sys.exit(
        f'Error: rust_toolchain must be a release like 1.100.0 (got {RUST_TOOLCHAIN!r})'
    )
# the post-generation hook formats the Rust sources with cargo
if shutil.which('cargo') is None:
    sys.exit('Error: cargo is required (https://rustup.rs)')
