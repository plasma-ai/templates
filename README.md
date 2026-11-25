# templates

[![license](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Templates for Plasma repositories.

Uses [Cruft](https://cruft.github.io/cruft/) for downstream synchronization —
when the template changes, projects created from it can pull updates via
`cruft check` and `cruft update`.

## Templates

| Template          | Description                                                                                                                  |
| :---------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| [python](python/) | Standalone Python package with CLI, skill definitions, plugin manifests, docs, and Plasma conventions                        |
| [rust](rust/)     | Command-line tool in Rust, shipped as a binary wheel, with skill definitions, plugin manifests, docs, and Plasma conventions |

`init.sh` creates a repository from a template:

```bash
bash templates/init.sh <name> --template=<template> --context=<path> --org=<org>
```

See the README in each template subdirectory for the context file and what the
template renders.

## License

Licensed under the Apache License 2.0 — see [LICENSE](LICENSE).
