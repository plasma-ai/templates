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

## Creating a Repository

1. Create the empty repository on GitHub. `init.sh` pushes the first commit to
   it.

2. Register the services the workflows use: the repository with Codecov, and the
   docs project on Read the Docs. The template's README says which Codecov token
   goes where.

3. Write the JSON context file. The template's README lists its fields.

4. Run `init.sh` from the parent of where the project will be created:

   ```bash
   bash templates/init.sh <name> --template=<template> --context=<path> --org=<org>
   ```

   It renders the template with cruft, renames the scaffold when the repository
   name differs from `project.name`, links `CLAUDE.md` to `AGENTS.md`, generates
   `Cargo.lock` for a Rust tool, and makes one commit on the context's branch,
   which it pushes. `--local` skips the push.

See the README in each template subdirectory for the context file and what the
template renders.

## License

Licensed under the Apache License 2.0 — see [LICENSE](LICENSE).
