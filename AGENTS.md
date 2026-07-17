# AGENTS

This file provides guidance to coding agents (Claude Code, Codex) when
working with code in this repository. If you are not Claude Code (which
already reads parent directories), also check the parent directory for
`AGENTS.md`.

## Overview

This repository hosts the org's `cookiecutter` package template:
`package/` is the canonical boilerplate every org package derives from.

Files under `package/` are cookiecutter output — keep every `{{ ... }}`
placeholder and `{% raw %}`/`{% endraw %}` guard intact, and remember
that edits here propagate to every derived repository via
`cruft update`.

## Build & Development

The repository is cookiecutter sources, so there is no build step. Run
pre-commit to format and lint the repo's own docs — the `package/`
templates are excluded (their `{{ ... }}` placeholders are not valid to
reformat):

```bash
pre-commit run --all-files
```

Install the hooks once with `pre-commit install` so they also run on
each commit.
