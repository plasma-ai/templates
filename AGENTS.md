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

```bash
# run pre-commit
pre-commit run --all-files
```
