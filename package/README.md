# Package Template

Template for Plasma packages.

## Prerequisites

- Python 3.11+
- [Cruft](https://cruft.github.io/cruft/)
- Git

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

## Codecov

1. Go to `codecov.io/gh/<org>` and add the repository.
2. Copy the *graphing token* — you will need it for the
   `project.codecov.token` field in the context file below.
3. Copy the *secret token* into GitHub Actions secrets as
   `CODECOV_TOKEN`.

## Initialization

**Important:** Cruft clones the templates repo from GitHub. Commit and
push any template changes before scaffolding.

1. Create a JSON context file with your project details:

   ```json
   {
       "branch": "main",
       "python": ">=3.11,<3.15",
       "default_python": "3.14",
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
   - `python` — Python version constraint for `requires-python`
     (required)
   - `default_python` — Python version for `.python-version`, CI, and
     Read the Docs (required)
   - `project.name` — Python package name (directory and import name)
   - `project.package` — PyPI package name (for install commands and
     badges)
   - `project.title` — human-readable project title
   - `project.description` — one-line description
   - `project.version` — initial version string
   - `project.authors` / `project.maintainers` — list of `{name, email}`
     objects
   - `project.codecov.token` — Codecov graphing token for badge
     rendering
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

   - `name` is the repository name — the scaffold directory is renamed
     to it when it differs from `project.name` (e.g. an import package
     with underscores).
   - `context` is the path to the JSON context file.
   - `org` is the GitHub organization name.

## Sphinx

Generate API documentation (`<package>` is `project.name`, the import
package directory):

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
