---
name: {{ cookiecutter.project.name }}
description: "{{ cookiecutter.project.description }}"
disable-model-invocation: true
---

# {{ cookiecutter.project.title }}

{{ cookiecutter.project.description }}

## Usage

Install the CLI from PyPI if it is not already on your `PATH`:

```bash
pipx install {{ cookiecutter.project.package }}
```

(`pip install` or `uv tool install` work too.)

Then run commands directly:

```bash
{{ cookiecutter.project.name }} <command> ...
```

Run `{{ cookiecutter.project.name }} --help` for a list of commands, and
`{{ cookiecutter.project.name }} <command> --help` for full option
descriptions.
