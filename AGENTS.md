# AGENTS

This file provides guidance to coding agents (Claude Code, Codex) when working
with code in this repository. If you are not Claude Code (which already reads
parent directories), also check the parent directory for `AGENTS.md`.

## Overview

This repository hosts the org's `cookiecutter` templates: `python/` is the
canonical boilerplate every org Python package derives from, and `rust/` the
canonical boilerplate for a command-line tool written in Rust.

Files under a template's `{{ cookiecutter.project.name }}/` folder are
cookiecutter output — keep every `{{ ... }}` placeholder and
`{% raw %}`/`{% endraw %}` guard intact, and remember that edits here propagate
to every derived repository via `cruft update`.

Each template folder is self-contained: cruft renders one folder, and a
project's lint job compares only its own folder with the template's head. Keep
symlinks, includes and hooks from reaching outside a folder.

A change to text both templates share goes to both folders in one commit;
`check.sh`, a pre-commit hook, fails when that text differs.

## Template Files

Scaffolding is non-interactive: `init.sh` passes a reviewed JSON context file to
`cruft create --no-input`, and cruft keeps the context in the project's
`.cruft.json` for later updates.

The context in each `cookiecutter.json` is nested objects that mirror config
domains (`project`, `github`, `urls`), with empty-string defaults except
`branch`. Keep a key that `cruft update --variables-to-update` sets on its own,
such as `rust_toolchain`, at the top level: the command replaces each top-level
value whole, so passing one nested key would reset the rest of its object to the
defaults.

In a rendered file:

- Compute a derived value once, with a set-block at the top of the file.
- Wrap a whole shell script in `{%- raw %}...{%- endraw %}`, and each GitHub
  Actions `${{ }}` expression in `{% raw %}...{% endraw %}`. A raw guard is
  mandatory wherever the file's own syntax collides with Jinja's delimiters.
- Make optional metadata conditional, and render list-valued metadata with
  for-loops that stay valid when the list is empty.

## Build & Development

The repository is cookiecutter sources, so there is no build step. Run
pre-commit to format and lint the repo — paths containing `{{ ... }}`
placeholders skip the parsing hooks (JSON/YAML checkers, mdformat, and the shell
hooks; placeholders are not valid to parse or reformat) but keep the safety
hooks (forbidden files, whitespace, merge conflicts, private keys); everything
else (`init.sh`, `check.sh`, and each template's `README.md` and
`cookiecutter.json`) is fully checked:

```bash
pre-commit run --all-files
```

Install the hooks once with `pre-commit install` so they also run on each
commit.

Derived repositories run mdformat on the Markdown this repository skips, so wrap
templated prose at 80 columns, and keep placeholders out of wrapped prose, where
their length would move the line breaks.
