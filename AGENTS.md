# Template Repository

This repository is the template that new repositories of Aonyx start from. It
holds no product code. It carries the configuration, the tooling, and the
conventions that every repository shares. A repository that grows out of the
template replaces this section with a description of its own project.

## Engineering

Architecture Decision Records (ADRs) in `adrs/` document why the project is
shaped the way it is. [ADR-001][adr-001] defines the process.

- Before you start a task, read the ADRs that relate to it.
- If your task conflicts with an ADR, stop and report the conflict. Do not work
  around the ADR.
- Do not edit an accepted ADR, except to change its status.
- If a task needs a decision that is architectural, cross-cutting, or hard to
  reverse, draft an ADR before you implement the task. Start from
  `adrs/000-template.md`, give it the next free number, and open it in its own
  pull request. A new ADR can also supersede an existing one.

## Language

- Use American English spelling, e.g. "color" not "colour".

## Markdown

- Use title case in headings and titles.
- Always use the Oxford comma.
- Use reference-style Markdown links, not inline links.
- Table cells must be single-line. Markdown does not support multi-line cells;
  each newline starts a new row. Ignore line length limits for table rows.

## Tooling

[Mise] provisions every external tool at the version that `mise.toml` pins.
Run `mise install` once, and activate mise in your shell so that the tools and
the stubs in `bin` reach your `PATH`.

[Rakko] provides the maintenance tasks. `tools/rakko` is the harness of this
repository: a package that mounts the bundles of actions this repository runs,
and that turns each action into a command. Run `mise run rakko` to list the
commands, or `rakko` where mise supplies its environment.

- Add an action by mounting it in `tools/rakko/src/main.rs`, and pin the tool
  it needs in `mise.toml`. Rakko installs nothing, and an action whose tool
  mise does not report stops instead of passing quietly.
- CI enumerates the commands from the help of the harness and runs each one
  whose name starts with `build-`, `check-`, `format-`, `lint-`, or `test-`,
  so a mounted action with one of these prefixes becomes a required check.
- `mise run rakko -- pre-commit` runs the actions that guard a commit, and
  `--fix` lets the formatters repair what they find. The Git hook runs it.

## Version Control

- Never commit directly to `main`, always create a branch or worktree.
- Every commit should be a logical unit of change.
- Every commit must build and pass all checks. Use the harness for
  verification (e.g. `mise run rakko -- pre-commit`).
- Fixes and refactoring should be in separate commits from features.
- Each pull request should have one primary commit with a well-crafted
  message — this is what lands in the Git history. Follow-up fixups within
  the same PR can use simple one-liner messages since they get squashed into
  the primary commit on merge.
- Reuse the commit message as the pull request description, but reflow each
  paragraph onto one line, because GitHub renders every newline as a line
  break. Do not use `gh pr create --fill`.

### Commit Messages

- We use Git as our Version Control System and GitHub to host the code.
- We use pre-commit hooks to verify the changes before committing them.
- We follow this [style guide][git-style-guide] for commit messages:
  - Capitalized, short (50 characters or less) summary in imperative mode
    ("Fix bug", not "Fixed bug")
  - Blank line between summary and body
  - Focus on the "why" — motivation and reasoning — not what changed
  - Minimal formatting or bullet points, plain prose is preferred
  - Full sentences with simple past and present tense
  - Wrap the body at 72 characters
- Write commit messages for a reader that has no prior context and no access to
  the session history.
- Keep commit messages concise. Aim for two or three paragraphs, not more.
- Don't use backticks in commit message titles, but do use them in bodies.
- **Never** write conventional commit messages.
- **Never** add yourself as a co-author.

[adr-001]: adrs/001-adrs.md
[git-style-guide]: https://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html
[mise]: https://mise.jdx.dev
[rakko]: https://github.com/aonyx-ai/rakko
