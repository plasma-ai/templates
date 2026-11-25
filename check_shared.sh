#!/usr/bin/env bash
set -euo pipefail

# Check that the text the templates share has not drifted
# -------------------------------------------------------

cd "$(dirname "$0")"

PYTHON="python/{{ cookiecutter.project.name }}"
RUST="rust/{{ cookiecutter.project.name }}"
# files both templates render that differ on purpose
DIFFERS=(
    .github/workflows/build.yaml
    .github/workflows/docs.yaml
    .github/workflows/lint.yaml
    .github/workflows/tests.yaml
    .gitignore
    .pre-commit-config.yaml
    .readthedocs.yaml
    AGENTS.md
    README.md
    docs/conf.py
    docs/index.rst
    pyproject.toml
)
STATUS=0

fail() {
    echo "Error: $1" >&2
    STATUS=1
}

# print the lines of a file from one pattern through another
span() {
    sed -n "/$2/,/$3/p" "$1"
}

# ------ whole files

# a file both templates render is identical unless it is listed above
while IFS= read -r -d '' FILE; do
    NAME="${FILE#"$PYTHON"/}"
    [[ -f "$RUST/$NAME" ]] || continue
    for DIFFER in "${DIFFERS[@]}"; do
        [[ "$NAME" == "$DIFFER" ]] && continue 2
    done
    cmp -s "$FILE" "$RUST/$NAME" || fail "$NAME differs between python/ and rust/"
done < <(find "$PYTHON" -type f -print0)

# ------ shared spans of files that differ

# the org-wide sections of AGENTS.md: the header, and Consistency onwards
if ! diff -q \
    <(span "$PYTHON/AGENTS.md" '^# AGENTS$' '^## Overview$') \
    <(span "$RUST/AGENTS.md" '^# AGENTS$' '^## Overview$') >/dev/null; then
    fail "the header of AGENTS.md differs between python/ and rust/"
fi
if ! diff -q \
    <(span "$PYTHON/AGENTS.md" '^## Consistency$' '^## Code Style$') \
    <(span "$RUST/AGENTS.md" '^## Consistency$' '^## Code Style$') >/dev/null; then
    fail "AGENTS.md differs between python/ and rust/ from Consistency to Code Style"
fi

# the pre-commit baseline: the Rust config is the Python one plus its own block
SIZE=$(wc -c <"$PYTHON/.pre-commit-config.yaml")
if ! head -c "$SIZE" "$RUST/.pre-commit-config.yaml" | cmp -s - "$PYTHON/.pre-commit-config.yaml"; then
    fail ".pre-commit-config.yaml of rust/ does not start with that of python/"
fi

# the ruff settings, up to the per-file ignores
if ! diff -q \
    <(span "$PYTHON/pyproject.toml" '^\[tool\.ruff\]$' '^\[tool\.ruff\.lint\.per-file-ignores\]$') \
    <(span "$RUST/pyproject.toml" '^\[tool\.ruff\]$' '^\[tool\.ruff\.lint\.per-file-ignores\]$') >/dev/null; then
    fail "the ruff settings in pyproject.toml differ between python/ and rust/"
fi

exit "$STATUS"
