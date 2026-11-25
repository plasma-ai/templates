# Rust Template

Template for Plasma command-line tools written in Rust and shipped on PyPI as a
binary wheel.

## Prerequisites

- [rustup](https://rustup.rs)
- [Cruft](https://cruft.github.io/cruft/)
- [uv](https://docs.astral.sh/uv/)
- Git

Rendering the template runs `cargo fmt`, so `cruft create`, `cruft update` and
`cruft diff` all need cargo and the toolchain the context pins.

## What You Get

```text
<tool>/
├── .claude-plugin/        Claude Code plugin manifest
├── .codex-plugin/         Codex plugin manifest
├── .config/
│   └── nextest.toml       Test runner profile for CI
├── .github/workflows/     CI pipelines (build, docs, lint, tests)
├── crates/
│   ├── <tool>-cli/        Command-line interface, built as the binary
│   └── <tool>-core/       Domain library, with the skill installer
├── docs/                  Sphinx documentation
├── examples/              Example projects
├── <tool>/
│   └── skills/            Skill definitions, embedded in the binary
│       └── <tool>/
│           ├── SKILL.md
│           └── agents/    Per-agent config
├── .gitattributes
├── .gitignore
├── .pre-commit-config.yaml
├── .python-version
├── .readthedocs.yaml
├── AGENTS.md
├── CLAUDE.md -> AGENTS.md
├── Cargo.lock
├── Cargo.toml             Workspace manifest: versions, lints, profiles
├── clippy.toml            Clippy configuration
├── codecov.yaml
├── deny.toml              Dependency policy
├── install.sh
├── pyproject.toml         Wheel metadata and the maturin build
├── README.md
├── rust-toolchain.toml    Pinned toolchain
└── rustfmt.toml           Formatter configuration
```

The binary starts with `--version`, an `install` command for the skill and a
`completion` command. The wheel holds the binary and nothing else: the project
has no Python package.

## Codecov

1. Go to `codecov.io/gh/<org>` and add the repository.
2. Copy the *graphing token* — you will need it for the `project.codecov.token`
   field in the context file below.
3. Copy the *secret token* into GitHub Actions secrets as `CODECOV_TOKEN`.

## Initialization

**Important:** Cruft clones the templates repo from GitHub. Commit and push any
template changes before scaffolding.

1. Create a JSON context file with your project details:

   ```json
   {
       "branch": "main",
       "python": ">=3.11",
       "default_python": "3.14",
       "rust_toolchain": "1.100.0",
       "project": {
           "name": "tool_name",
           "package": "package-name",
           "title": "Tool Name",
           "description": "Description of tool.",
           "version": "0.0.0",
           "authors": [{"name": "...", "email": "..."}],
           "maintainers": [{"name": "...", "email": "..."}],
           "codecov": {"token": "..."}
       },
       "github": {
           "org": "...",
           "user": "...",
           "repo": "repo-name"
       },
       "urls": {
           "domain": "...",
           "docs": "..."
       }
   }
   ```

   Fields:

   - `branch` — default git branch (defaults to `main`)
   - `python` — Python version constraint for `requires-python` (required); the
     wheel runs no Python, so give a floor with no cap
   - `default_python` — Python version for `.python-version`, CI, and Read the
     Docs (required)
   - `rust_toolchain` — exact stable Rust release for `rust-toolchain.toml` and
     `rust-version`, e.g. `1.100.0` (required)
   - `project.name` — tool name, in lowercase letters, digits and underscores:
     the binary, the skill folder, and with dashes the crate prefix
   - `project.package` — PyPI package name (for install commands and badges)
   - `project.title` — human-readable project title
   - `project.description` — one-line description
   - `project.version` — initial version string, in Cargo's spelling (`0.0.0`,
     `1.2.0-rc.1`)
   - `project.authors` / `project.maintainers` — list of `{name, email}` objects
   - `project.codecov.token` — Codecov graphing token for badge rendering
   - `github.org` — GitHub organization (used for repository URLs)
   - `github.user` — GitHub username (fallback when `org` is empty)
   - `github.repo` — repository name
   - `urls.domain` — base domain (optional, used in plugin manifests)
   - `urls.docs` — documentation host and path without scheme, e.g.
     `docs.example.com/name` (optional, enables Documentation link)

2. Run `init.sh` from the parent of where the project will be created:

   ```bash
   bash templates/init.sh <name> --template=rust --context=<path> --org=<org>
   ```

   - `name` is the repository name — the scaffold directory is renamed to it
     when it differs from `project.name`.
   - `template` is this template's folder, `rust`.
   - `context` is the path to the JSON context file.
   - `org` is the GitHub organization name.

   `init.sh` also runs `cargo generate-lockfile`, so the initial commit carries
   `Cargo.lock`.

## Cruft

Check for upstream template changes:

```bash
uv sync --inexact --group lint
uv run --no-sync cruft check
uv run --no-sync cruft update
```

Raise the pinned toolchain by changing the context, then refresh and commit the
lockfile:

```bash
uv run --no-sync cruft update --variables-to-update '{"rust_toolchain": "<X.Y.Z>"}'
cargo update
```
