#!/usr/bin/env bash
set -euo pipefail

# Check the templates
# -------------------

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

# compare the lines from one pattern through another in a file of both
# templates; an empty span fails too, since it would compare as equal
compare_span() {
    local FILE="$1"
    local START="$2"
    local END="$3"
    local LABEL="$4"
    local PYTHON_SPAN RUST_SPAN
    PYTHON_SPAN=$(sed -n "/$START/,/$END/p" "$PYTHON/$FILE")
    RUST_SPAN=$(sed -n "/$START/,/$END/p" "$RUST/$FILE")
    if [[ -z "$PYTHON_SPAN" || -z "$RUST_SPAN" ]]; then
        fail "$LABEL is missing from python/ or rust/"
    elif [[ "$PYTHON_SPAN" != "$RUST_SPAN" ]]; then
        fail "$LABEL differs between python/ and rust/"
    fi
}

# ------ shared files

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

# the org-wide sections of AGENTS.md: the header, and Consistency up to Code Style
compare_span AGENTS.md '^# AGENTS$' '^## Overview$' "the header of AGENTS.md"
compare_span AGENTS.md '^## Consistency$' '^## Code Style$' \
    "AGENTS.md from Consistency to Code Style"

# the pre-commit baseline: the Rust config is the Python one plus its own block
SIZE=$(wc -c <"$PYTHON/.pre-commit-config.yaml")
if ! head -c "$SIZE" "$RUST/.pre-commit-config.yaml" | cmp -s - "$PYTHON/.pre-commit-config.yaml"; then
    fail ".pre-commit-config.yaml of rust/ does not start with that of python/"
fi

# the ruff settings, apart from the per-file ignores
compare_span pyproject.toml '^\[tool\.ruff\]$' '^\[tool\.ruff\.lint\.per-file-ignores\]$' \
    "the ruff configuration of pyproject.toml"
compare_span pyproject.toml '^\[tool\.ruff\.lint\.isort\]$' '^suppress-dummy-args = ' \
    "the ruff configuration after the per-file ignores of pyproject.toml"

exit "$STATUS"
