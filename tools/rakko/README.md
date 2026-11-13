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

- [`rakko-baseline`][rakko-baseline] carries the formatters and the linters
  for the files that any repository holds, whatever language it is written in.
- [`rakko-rust-binary`][rakko-rust-binary] carries everything that a Rust
  project runs. The harness is the only Rust in the template, so the bundle
  formats, lints, and tests the harness itself.

A project that grows out of this template and publishes a library mounts
[`rakko-rust-library`][rakko-rust-library] instead of `rakko-rust-binary`. The
library bundle contains everything that the binary bundle carries, and a
harness stops when two bundles give it two actions of one name.

## Commands

### `pre-commit`

The command comes from [`rakko-pre-commit`][rakko-pre-commit]. It runs the
actions that guard a commit: the formatters first, in the order in which they
rewrite the tree, and then the checks that read what they wrote. `src/main.rs`
gives the command one list of these actions, and the README of the crate
describes how a run orders them, and when it fails.

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

Rakko publishes no release to crates.io yet, so each dependency names the
repository and the revision that this harness builds. A revision moves in one
commit, which is what a review of the update reads.

[rakko]: https://github.com/aonyx-ai/rakko
[rakko-baseline]: https://github.com/aonyx-ai/rakko/blob/main/bundles/rakko-baseline/README.md
[rakko-pre-commit]: https://github.com/aonyx-ai/rakko/blob/main/commands/rakko-pre-commit/README.md
[rakko-rust-binary]: https://github.com/aonyx-ai/rakko/blob/main/bundles/rakko-rust-binary/README.md
[rakko-rust-library]: https://github.com/aonyx-ai/rakko/blob/main/bundles/rakko-rust-library/README.md
