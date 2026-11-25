# AGENTS

This file provides guidance to coding agents (Claude Code, Codex) when working
with code in this repository. If you are not Claude Code (which already reads
parent directories), also check the parent directory for `AGENTS.md`.

## Overview

This repository hosts the org's `cookiecutter` templates: `python/` is the
boilerplate every org Python package derives from, and `rust/` the boilerplate
for a command-line tool written in Rust.

Files under a template's `{{ cookiecutter.project.name }}/` folder are
cookiecutter output — keep every `{{ ... }}` placeholder and
`{% raw %}`/`{% endraw %}` guard intact, and remember that edits here propagate
to every derived repository via `cruft update`.

Each template folder is self-contained: cruft renders one folder, and a
project's lint job compares only its own folder with the template's head. Keep
symlinks, includes and hooks from reaching outside a folder.

A change to text both templates share goes to both folders in one commit.
`check_shared.sh`, a pre-commit hook, fails when a file both render differs
without being on its list, or when a shared section of `AGENTS.md`, the
pre-commit baseline or the ruff settings differs.

`rust/hooks/post_gen_project.py` runs `cargo fmt` on the rendered sources,
because rustfmt orders imports and breaks lines by the rendered project name,
which the template's text cannot follow.

## Build & Development

The repository is cookiecutter sources, so there is no build step. Run
pre-commit to format and lint the repo — paths containing `{{ ... }}`
placeholders skip the parsing hooks (JSON/YAML checkers, mdformat, and the shell
hooks; placeholders are not valid to parse or reformat) but keep the safety
hooks (forbidden files, whitespace, merge conflicts, private keys); everything
else (`init.sh`, `check_shared.sh`, and each template's `README.md`,
`cookiecutter.json` and `hooks/`) is fully checked:

```bash
pre-commit run --all-files
```

Install the hooks once with `pre-commit install` so they also run on each
commit.

Derived repositories run mdformat on the Markdown this repository skips, so wrap
templated prose at 80 columns, and keep placeholders out of wrapped prose, where
their length would move the line breaks.
