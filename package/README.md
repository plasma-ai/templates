# Package Template

Template for Plasma packages.

## Prerequisites

- Python 3.13+
- [Cruft](https://cruft.github.io/cruft/)
- Git

## What You Get

```
<package>/
├── .claude-plugin/        Claude Code plugin manifest
├── .codex-plugin/         Codex plugin manifest
├── .github/workflows/     CI pipelines (build, docs, lint, tests)
├── docs/                  Sphinx documentation
├── examples/              Example projects
├── <package>/             Python package
│   ├── __init__.py
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

**Important:** Cruft clones from the committed state of the templates
repo. Commit any template changes before scaffolding.

1. Create a JSON context file with your project details:

   ```json
   {
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
   - `urls.docs` — documentation URL (optional, enables Documentation
     link)

2. Run `init.sh` from the parent of where the project will be created:

   ```bash
   bash templates/package/init.sh <name> --context=<path> --org=<org>
   ```

   - `name` must match `project.name` in the context file.
   - `context` is the path to the JSON context file.
   - `org` is the GitHub organization name.

## Sphinx

Generate API documentation:

```bash
sphinx-apidoc -f -o docs <name>
```

Re-run when new modules are added.

## Cruft

Check for upstream template changes:

```bash
cruft check
cruft update
```
