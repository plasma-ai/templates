---
name: {{ cookiecutter.project.name }}
description: {{ cookiecutter.project.description }}
disable-model-invocation: true
---

# {{ cookiecutter.project.title }}

{{ cookiecutter.project.description }}

## Usage

Install the `{{ cookiecutter.project.name }}` CLI from PyPI if it is not already on your `PATH`:

```bash
pipx install {{ cookiecutter.project.package }}
```

(`pip install {{ cookiecutter.project.package }}` or `uv tool install {{ cookiecutter.project.package }}` work too.)

Run `{{ cookiecutter.project.name }} --help` for a list of commands, and
`{{ cookiecutter.project.name }} <command> --help` for full option descriptions.
