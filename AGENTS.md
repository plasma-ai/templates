# AGENTS

This file provides guidance to coding agents (Claude Code, Codex) when working
with code in this repository. If you are not Claude Code (which already reads
parent directories), also check the parent directory for `AGENTS.md`.

## Overview

This repository hosts the org's `cookiecutter` package template: `package/` is
the canonical boilerplate every org package derives from.

Files under `package/` are cookiecutter output — keep every `{{ ... }}`
placeholder and `{% raw %}`/`{% endraw %}` guard intact, and remember that edits
here propagate to every derived repository via `cruft update`.

## Build & Development

The repository is cookiecutter sources, so there is no build step. Run
pre-commit to format and lint the repo — paths containing `{{ ... }}`
placeholders skip the parsing hooks (JSON/YAML checkers, mdformat, and the shell
hooks; placeholders are not valid to parse or reformat) but keep the safety
hooks (forbidden files, whitespace, merge conflicts, private keys); everything
else in `package/` (README.md, init.sh, cookiecutter.json) is fully checked:

```bash
pre-commit run --all-files
```

Install the hooks once with `pre-commit install` so they also run on each
commit.
