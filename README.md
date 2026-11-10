# Template Repository

This repository is a template for creating new repositories. It comes
preconfigured with a default developer experience, including pre-commit hooks
and GitHub Actions.

## Usage

When creating a new repository in [otterbuild/den], set `from_template` to
`true` to create the repository from this template.

```terraform
from_template = true
```

## Development

[Mise] provisions every tool that this repository needs, at the versions that
`mise.toml` pins:

```console
mise install
```

Activate mise in your shell, as the [mise documentation][mise-activate]
describes. Activation puts the pinned tools on your `PATH`, and with them the
`bin` directory of the repository, which holds a stub for every mise task.

[Rakko] provides the maintenance tasks. The package in `tools/rakko` is the
harness: it mounts the actions that maintain this repository, and it turns
each of them into a command. A run without a command lists them:

```console
mise run rakko
```

Where mise supplies its environment, `rakko` is the same command from every
directory of the repository.

`pre-commit install` installs the Git hook, which runs
`rakko pre-commit --fix` before every commit.

[mise]: https://mise.jdx.dev
[mise-activate]: https://mise.jdx.dev/getting-started.html#activate-mise
[otterbuild/den]: https://github.com/otterbuild/den
[rakko]: https://github.com/aonyx-ai/rakko
