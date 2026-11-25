# AGENTS

This file provides guidance to coding agents (Claude Code, Codex) when working
with code in this repository. If you are not Claude Code (which already reads
parent directories), also check the parent directory for `AGENTS.md`.

## Overview

This repository hosts the org's `cookiecutter` package template: `python/` is
the canonical boilerplate every org Python package derives from.

Files under `python/` are cookiecutter output — keep every `{{ ... }}`
placeholder and `{% raw %}`/`{% endraw %}` guard intact, and remember that edits
here propagate to every derived repository via `cruft update`.

## Build & Development

The repository is cookiecutter sources, so there is no build step. Run
pre-commit to format and lint the repo — paths containing `{{ ... }}`
placeholders skip the parsing hooks (JSON/YAML checkers, mdformat, and the shell
hooks; placeholders are not valid to parse or reformat) but keep the safety
hooks (forbidden files, whitespace, merge conflicts, private keys); everything
else (`init.sh`, and `README.md` and `cookiecutter.json` in `python/`) is fully
checked:

```bash
pre-commit run --all-files
```

Install the hooks once with `pre-commit install` so they also run on each
commit.

Derived repositories run mdformat on the Markdown this repository skips, so wrap
templated prose at 80 columns, and keep placeholders out of wrapped prose, where
their length would move the line breaks.
