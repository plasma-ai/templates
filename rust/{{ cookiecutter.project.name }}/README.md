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

Install the `{{ cookiecutter.project.name }}` command from PyPI:

```bash
pip install {{ cookiecutter.project.package }}
```

Use `pipx install` or `uv tool install` to install the command in an isolated
environment.

On a platform with no ready-built wheel, the install builds from source, which
needs a Rust toolchain.

### Skill

Install the skill for your agent via the plugin marketplace (Claude Code and
Codex):

```bash
# Claude Code
/plugin marketplace add plasma-ai/plugins
/plugin install {{ cookiecutter.project.name }}@plasma

# Codex
codex plugin marketplace add plasma-ai/plugins
codex plugin add {{ cookiecutter.project.name }}@plasma
```

Another install route is from the CLI, which copies the skill into
`~/.claude/skills` and `~/.agents/skills` (add `--project` for the current
project only):

```bash
{{ cookiecutter.project.name }} install
```

After upgrading the package, re-run the install command to refresh the copied
skill.

## Usage

List the commands, or a command's options:

```bash
{{ cookiecutter.project.name }} --help
{{ cookiecutter.project.name }} <command> --help
```

## Development

### Install

The workspace needs [rustup](https://rustup.rs). Install the toolchain pinned in
`rust-toolchain.toml`:

```bash
rustup toolchain install
```

The tests need cargo-nextest, and the supply-chain checks need cargo-deny and
cargo-shear. Install each at the version the workflows pin.

Run `install.sh` in the repository root. With no environment active it creates
and uses a local `.venv`; with one active (e.g. pyenv) it installs into that
environment, without recreating it:

```bash
./install.sh --groups=lint,docs
```

Run `./install.sh --help` for all options. Alternatively, run
`uv sync --group lint --group docs` and `uv run pre-commit install` to set up
the environment manually.

The install compiles the binary into the environment. After a change under
`crates/`, re-run it to rebuild the binary, or run the change directly with
`cargo run`.

With a binary built in a development clone, the install command's `--link`
option symlinks the skill folder of that clone into the agent skill directories
instead of copying it, so skill edits apply without re-running the install.

Once installed, run tools with `uv run --no-sync <command>`, or activate the
environment first (`source .venv/bin/activate`).

### Tests

Run the test suite:

```bash
cargo nextest run
cargo test --doc
```

### Linting

Run linters and formatters:

```bash
pre-commit run --all-files
```

### Contributing

The contribution workflow, repository conventions, and release process (version
sources, tagging, CI guard) are documented in:

- Contribution workflow (organization-wide):
  [CONTRIBUTING.md](https://github.com/{{ github_owner }}/.github/blob/main/CONTRIBUTING.md)
- Repository conventions:
  [AGENTS.md](https://github.com/{{ github_owner }}/{{ cookiecutter.github.repo }}/blob/{{ cookiecutter.branch }}/AGENTS.md)
- Release process (organization-wide):
  [RELEASING.md](https://github.com/{{ github_owner }}/.github/blob/main/RELEASING.md)
