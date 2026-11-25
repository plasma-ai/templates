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

# compare the lines from one pattern through another in a file of both
# templates; an empty span fails too, since it would compare as equal
compare_span() {
    local ONE TWO
    ONE=$(sed -n "/$2/,/$3/p" "$PYTHON/$1")
    TWO=$(sed -n "/$2/,/$3/p" "$RUST/$1")
    if [[ -z "$ONE" || -z "$TWO" ]]; then
        fail "$4 is missing from $1 in python/ or rust/"
    elif [[ "$ONE" != "$TWO" ]]; then
        fail "$4 differs between python/ and rust/"
    fi
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
compare_span AGENTS.md '^# AGENTS$' '^## Overview$' "the header of AGENTS.md"
compare_span AGENTS.md '^## Consistency$' '^## Code Style$' \
    "AGENTS.md from Consistency to Code Style"

# the pre-commit baseline: the Rust config is the Python one plus its own block
SIZE=$(wc -c <"$PYTHON/.pre-commit-config.yaml")
if ! head -c "$SIZE" "$RUST/.pre-commit-config.yaml" | cmp -s - "$PYTHON/.pre-commit-config.yaml"; then
    fail ".pre-commit-config.yaml of rust/ does not start with that of python/"
fi

# the ruff settings, up to the per-file ignores
compare_span pyproject.toml '^\[tool\.ruff\]$' '^\[tool\.ruff\.lint\.per-file-ignores\]$' \
    "the ruff settings of pyproject.toml"

exit "$STATUS"
