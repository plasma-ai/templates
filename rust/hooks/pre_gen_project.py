"""Reject a context the Rust template cannot render."""

from __future__ import annotations

import re
import shutil
import sys

RUST_TOOLCHAIN = '{{ cookiecutter.rust_toolchain }}'

# the toolchain pin is one exact stable release
if re.fullmatch(r'\d+\.\d+\.\d+', RUST_TOOLCHAIN) is None:
    sys.exit(
        f'rust_toolchain must be a release like 1.100.0, got {RUST_TOOLCHAIN!r} instead.'
    )
# the post-generation hook formats the Rust sources with cargo
if shutil.which('cargo') is None:
    sys.exit('Please install cargo (https://rustup.rs): the template needs it.')
