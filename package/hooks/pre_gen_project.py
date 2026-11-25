"""Reject a context whose Rust settings the template cannot render."""

from __future__ import annotations

import shutil
import sys

RUST_TOOLCHAIN = '{{ cookiecutter.rust_toolchain }}'
RUST_BINDINGS = '{{ cookiecutter.rust_bindings }}'
BINDINGS = ['', 'bin']

# the wheel shape is one the template renders
if RUST_BINDINGS not in BINDINGS:
    sys.exit(f'rust_bindings must be one of {BINDINGS}, got {RUST_BINDINGS!r} instead.')
# a binary wheel is built from the Cargo workspace
if RUST_BINDINGS and not RUST_TOOLCHAIN:
    sys.exit('Please set rust_toolchain: rust_bindings needs the Cargo workspace.')
# the post-generation hook formats the Rust sources with cargo
if RUST_BINDINGS and shutil.which('cargo') is None:
    sys.exit('Please install cargo (https://rustup.rs): rust_bindings needs it.')
