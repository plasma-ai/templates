"""Format the rendered Rust sources."""

from __future__ import annotations

import subprocess

# the import layout the cargo-fmt pre-commit hook passes to rustfmt
RUSTFMT_CONFIG = 'group_imports=StdExternalCrate,imports_granularity=Item'

# rustfmt orders imports and breaks lines by the rendered project name, so
# the sources are formatted here, as the pre-commit hook would
subprocess.run(
    ['cargo', 'fmt', '--all', '--', '--config', RUSTFMT_CONFIG],
    check=True,
)
