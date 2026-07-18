{%- set github_owner = cookiecutter.github.org or cookiecutter.github.user -%}
# {{ cookiecutter.project.name }}

[![build](https://github.com/{{ github_owner }}/{{ cookiecutter.github.repo }}/actions/workflows/build.yaml/badge.svg)](https://github.com/{{ github_owner }}/{{ cookiecutter.github.repo }}/actions/workflows/build.yaml)
[![docs](https://github.com/{{ github_owner }}/{{ cookiecutter.github.repo }}/actions/workflows/docs.yaml/badge.svg)](https://github.com/{{ github_owner }}/{{ cookiecutter.github.repo }}/actions/workflows/docs.yaml)
[![lint](https://github.com/{{ github_owner }}/{{ cookiecutter.github.repo }}/actions/workflows/lint.yaml/badge.svg)](https://github.com/{{ github_owner }}/{{ cookiecutter.github.repo }}/actions/workflows/lint.yaml)
[![tests](https://github.com/{{ github_owner }}/{{ cookiecutter.github.repo }}/actions/workflows/tests.yaml/badge.svg)](https://github.com/{{ github_owner }}/{{ cookiecutter.github.repo }}/actions/workflows/tests.yaml)
{%- if cookiecutter.project.codecov.token %}
[![codecov](https://codecov.io/gh/{{ github_owner }}/{{ cookiecutter.github.repo }}/branch/{{ cookiecutter.branch }}/graph/badge.svg?token={{ cookiecutter.project.codecov.token }})](https://codecov.io/gh/{{ github_owner }}/{{ cookiecutter.github.repo }})
{%- endif %}
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit)

{{ cookiecutter.project.description }}

______________________________________________________________________

**Source**:
[https://github.com/{{ github_owner }}/{{ cookiecutter.github.repo }}](https://github.com/{{ github_owner }}/{{ cookiecutter.github.repo }})

**Package**:
[https://pypi.org/project/{{ cookiecutter.project.package }}/](https://pypi.org/project/{{ cookiecutter.project.package }}/)
{%- if cookiecutter.urls.docs %}

**Documentation**:
[https://{{ cookiecutter.urls.docs }}](https://{{ cookiecutter.urls.docs }})
{%- endif %}

______________________________________________________________________

## Installation

Install the `{{ cookiecutter.project.name }}` package from PyPI:

```bash
pip install {{ cookiecutter.project.package }}
```

Use `pipx install {{ cookiecutter.project.package }}` or `uv tool install {{ cookiecutter.project.package }}` to
install in an isolated environment.

### Skill

Install the `/{{ cookiecutter.project.name }}` skill for your agent via the plugin marketplace
(Claude Code and Codex):

```bash
# Claude Code
/plugin marketplace add plasma-ai/plugins
/plugin install {{ cookiecutter.project.name }}@plasma

# Codex
codex plugin marketplace add plasma-ai/plugins
codex plugin add {{ cookiecutter.project.name }}@plasma
```

Or from the CLI, which copies the skill into `~/.claude/skills` and
`~/.agents/skills` (add `--project` for the current project only):

```bash
{{ cookiecutter.project.name }} install
```

## Usage

Basic usage:

```python
import {{ cookiecutter.project.name }}
```

## Development

### Install

Run `install.sh` in the package root. With no environment active it
creates and uses a local `.venv`; with one active (e.g. pyenv) it
installs into that environment (editable), without recreating it:

```bash
./install.sh --all-extras --groups=test,lint,type
```

Run `./install.sh --help` for all options. Alternatively, run
`uv sync --all-extras --group test --group lint --group type` and
`uv run pre-commit install` to set up the environment manually.

Installing a dependency as editable (e.g. a sibling package) is left to
the caller: `uv pip install --editable <path>`.

Once installed, run tools with `uv run --no-sync <command>`, or activate
the environment first (`source .venv/bin/activate`).

### Tests

Run the test suite:

```bash
pytest .
```

### Linting

Run linters and formatters:

```bash
pre-commit run --all-files
```

### Contributing

The contribution workflow is covered by the organization-wide
[CONTRIBUTING.md](https://github.com/{{ github_owner }}/.github/blob/main/CONTRIBUTING.md);
repository conventions live in
[AGENTS.md](https://github.com/{{ github_owner }}/{{ cookiecutter.github.repo }}/blob/{{ cookiecutter.branch }}/AGENTS.md),
and the release process (version sources, tagging, CI guard) in the
organization-wide
[RELEASING.md](https://github.com/{{ github_owner }}/.github/blob/main/RELEASING.md).
