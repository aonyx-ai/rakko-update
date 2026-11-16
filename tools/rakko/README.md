# Maintenance Tasks

This crate implements a CLI with maintenance tasks for this repository. It can
be used to format, lint, and test the code in this repository, or perform any
other repetitive task.

```shell
mise run rakko --help
```

The CLI is built with [Rakko], our internal task runner. It pulls in shared
tasks, and can be easily extended by implementing project-specific tasks in
this crate.

## Bundles

The harness mounts two bundles:

- [`aonyx-rakko-baseline`][aonyx-rakko-baseline] carries the formatters and the
  linters for the files that any repository holds, whatever language it is
  written in.
- [`aonyx-rakko-rust-binary`][aonyx-rakko-rust-binary] carries everything that
  a Rust project runs. The harness is the only Rust in the template, so the
  bundle formats, lints, and tests the harness itself.

A project that grows out of this template and publishes a library mounts
[`aonyx-rakko-rust-library`][aonyx-rakko-rust-library] instead of
`aonyx-rakko-rust-binary`. The library bundle contains everything that the
binary bundle carries, and a harness stops when two bundles give it two actions
of one name.

## Commands

### `pre-commit`

The command comes from [`aonyx-rakko-pre-commit`][aonyx-rakko-pre-commit]. It
runs the actions that guard a commit: the formatters first, in the order in
which they rewrite the tree, and then the checks that read what they wrote.
`src/main.rs` gives the command one list of these actions, and the README of
the crate describes how a run orders them, and when it fails.

```console
mise run rakko -- pre-commit --fix
```

The hook that Git runs before a commit passes `--fix`, so that the formatters
rewrite the files that they can format. A run without the flag reports what a
commit has to repair, and it changes nothing.

The list leaves out the actions that resolve the dependencies of the project
again, such as `check-latest-deps`. Each of them reaches the network and takes
minutes, which belongs in a job and not in front of every commit. A
contributor runs them by name, and CI runs them on every pull request.

## Layout

The crate is a member of the Cargo workspace at the root of the repository,
so the Rust actions that the harness mounts check it like any other crate. The
binary is named `rakko`, and the package is named `harness`, because a package
that depends on the `rakko` crates cannot carry that name as well.

Each crate of Rakko comes from its release on crates.io. Renovate updates the
crates, and the workflow in `.github/workflows/rakko-update.yml` then writes
the tool pins that the new releases declare into the same pull request.

[aonyx-rakko-baseline]: https://github.com/aonyx-ai/rakko/blob/main/bundles/aonyx-rakko-baseline/README.md
[aonyx-rakko-pre-commit]: https://github.com/aonyx-ai/rakko/blob/main/commands/aonyx-rakko-pre-commit/README.md
[aonyx-rakko-rust-binary]: https://github.com/aonyx-ai/rakko/blob/main/bundles/aonyx-rakko-rust-binary/README.md
[aonyx-rakko-rust-library]: https://github.com/aonyx-ai/rakko/blob/main/bundles/aonyx-rakko-rust-library/README.md
[rakko]: https://github.com/aonyx-ai/rakko
