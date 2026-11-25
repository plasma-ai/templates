# Package Template

Template for Plasma packages.

## Prerequisites

- Python 3.11+
- [Cruft](https://cruft.github.io/cruft/)
- Git
- [rustup](https://rustup.rs), for a project with a Rust workspace

## What You Get

```text
<package>/
├── .claude-plugin/        Claude Code plugin manifest
├── .codex-plugin/         Codex plugin manifest
├── .github/workflows/     CI pipelines (build, docs, lint, tests)
├── docs/                  Sphinx documentation
├── examples/              Example projects
├── <package>/             Python package
│   ├── __init__.py
│   ├── constants.py       Constants
│   ├── exceptions.py      Exception classes
│   ├── typing.py          Type hints
│   ├── cli/               CLI layer
│   │   ├── main.py        CLI entry point
│   │   ├── utils.py       Shared CLI helpers
│   │   └── cmd/           Command modules
│   ├── core/              Core classes
│   ├── skills/            Skill definitions
│   │   └── <package>/
│   │       ├── SKILL.md
│   │       └── agents/    Per-agent config
│   └── util/              Utilities
├── tests/                 Test suite
├── .gitattributes
├── .gitignore
├── .pre-commit-config.yaml
├── .python-version
├── .readthedocs.yaml
├── AGENTS.md
├── CLAUDE.md -> AGENTS.md
├── codecov.yaml
├── install.sh
├── pyproject.toml
└── README.md
```

A context that sets `rust.toolchain` adds a Cargo workspace beside the Python
package:

```text
<package>/
├── .config/
│   └── nextest.toml       Test runner profile for CI
├── crates/
│   └── <package>-core/    Domain library, with its integration test binary
├── Cargo.toml             Workspace manifest: versions, lints, profiles
├── clippy.toml            Clippy configuration
├── deny.toml              Dependency policy
├── rust-toolchain.toml    Pinned toolchain
└── rustfmt.toml           Formatter configuration
```

It also adds the Rust block to `.gitignore`, the cargo hooks to
`.pre-commit-config.yaml`, and the Rust steps and jobs to the four workflows.

A context that also sets `rust.bindings` to `bin` makes the command a Rust
binary, which maturin builds into the wheel. The Python CLI and the rest of the
package skeleton give way to a launcher, and the workspace gains a CLI crate:

```text
<package>/
├── crates/
│   ├── <package>-cli/     Command-line interface, built as the binary
│   └── <package>-core/    Domain library, with the skill installer
├── <package>/             Python package
│   ├── __init__.py
│   ├── __main__.py        Launcher for `python -m <package>`
│   └── skills/            Skill definitions, embedded in the binary
└── tests/                 Test suite for the launcher
```

The binary starts with `--version`, an `install` command for the skill and a
`completion` command. `pyproject.toml` switches to the maturin backend with the
version read from `Cargo.toml`, and the build workflow builds one wheel per
platform and an sdist.

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
       "python": ">=3.11,<3.15",
       "default_python": "3.14",
       "rust": {"toolchain": "", "bindings": ""},
       "project": {
           "name": "project-name",
           "package": "package-name",
           "title": "Project Name",
           "description": "Description of project.",
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
   - `python` — Python version constraint for `requires-python` (required)
   - `default_python` — Python version for `.python-version`, CI, and Read the
     Docs (required)
   - `rust.toolchain` — exact stable Rust release for `rust-toolchain.toml` and
     `rust-version`, e.g. `1.100.0` (optional, adds the Cargo workspace)
   - `rust.bindings` — wheel shape, `bin` for a package whose command is a Rust
     binary (optional, needs `rust.toolchain`)
   - `project.name` — Python package name (directory and import name)
   - `project.package` — PyPI package name (for install commands and badges)
   - `project.title` — human-readable project title
   - `project.description` — one-line description
   - `project.version` — initial version string
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
   bash templates/package/init.sh <name> --context=<path> --org=<org>
   ```

   - `name` is the repository name — the scaffold directory is renamed to it
     when it differs from `project.name` (e.g. an import package with
     underscores).
   - `context` is the path to the JSON context file.
   - `org` is the GitHub organization name.

   With `rust.toolchain` set, `init.sh` also runs `cargo generate-lockfile`, so
   the initial commit carries `Cargo.lock`.

## Sphinx

Generate API documentation (`<package>` is `project.name`, the import package
directory):

```bash
uv sync --inexact --group docs
uv run --no-sync sphinx-apidoc -f -o docs <package>
```

Re-run when new modules are added.

## Cruft

Check for upstream template changes:

```bash
uv sync --inexact --group lint
uv run --no-sync cruft check
uv run --no-sync cruft update
```

Add the Cargo workspace to an existing project by setting the toolchain in its
context, then generate and commit the lockfile:

```bash
uv run --no-sync cruft update --variables-to-update '{"rust": {"toolchain": "<X.Y.Z>"}}'
cargo generate-lockfile
```

Set `rust.bindings` when a project is created: on an existing project it removes
the Python CLI the project has built.
