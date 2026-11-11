# Rakko Update

A GitHub Action that keeps an update of [Rakko] green in a project that
Dependabot or Renovate updates.

A project that uses Rakko has a harness: a small program that mounts the
actions of Rakko that the project runs, such as a formatter or a linter. Each
mounted action declares the tools that it runs, at an exact version. The
project commits the pins of these declared tools in
`.config/mise/conf.d/rakko.toml` and in its lockfile.
`mise run rakko -- doctor --fix` writes both files.

An update of Rakko can change a declared tool. The two files then no longer
match the mounted actions. Rakko then stops every command except the help and
`doctor`, until `doctor --fix` writes the files again.

This action updates the tool pins for you, in the pull request of an update bot:

1. It runs `mise run rakko -- doctor --fix` on the branch of the pull request.
2. It commits the changed files to the same branch.

The update and its tool pins then arrive in one pull request.

## What the Action Does

- On a pull request of a bot, such as Renovate or Dependabot, the action runs
  `doctor --fix`. When files change, it commits them to the branch of the pull
  request. When nothing changes, it commits nothing.
- On the pull request of a person, the action changes nothing. Rakko tells the
  person to run `doctor --fix`, and the person commits the result.
- A person can push a commit onto the branch of a bot. The action then changes
  nothing until the bot makes the last commit of the branch again.
- When the bot rebases its branch, the rebase removes the commit of the action.
  The action then commits again.
- When `doctor --fix` cannot update the tool pins, the action fails and pushes
  nothing. The checks of the pull request then fail. For example, an action of
  Rakko can need a newer toolchain than the project pins. The output of
  `doctor` says which pin to change.

### The Bots

The action updates a pull request only when all of these are true:

- An account in its list of bots opened the pull request.
- The branch of the pull request is in the same repository. So the action
  changes nothing on a pull request from a fork.
- The last commit of the branch comes from an account in the list. GitHub
  finds the account from the email address of the author of the commit.

The list always holds `renovate[bot]` and `dependabot[bot]`. To add the account
of another bot, use the input `bots`. Write one login on each line:

```yaml
- name: Update the tool pins
  uses: aonyx-ai/rakko-update@<commit> # main
  with:
    bots: |
      my-renovate[bot]
```

Add an account, for example, for a self-hosted Renovate, or for a Renovate
that runs as a GitHub App of your own. A self-hosted Renovate can open the pull
request with one account. It writes its commits with the `gitAuthor` of its
configuration, and GitHub can show that author as another account. List both
accounts. When the account of the pull request or of its last commit is not in
the list, the notice of the action gives that login.

The commits of the action come from `github-actions[bot]`, which is not in the
list. So after its own commit, the action changes nothing until the bot
updates its branch again.

## Usage

The project must run its harness as `mise run rakko`. It must use a revision
of Rakko that has the command `doctor`.

Before you add the action, run `mise run rakko -- doctor --fix` once. Then
commit the result.

Add a workflow, for example `.github/workflows/rakko-update.yml`:

```yaml
---
name: Rakko

"on":
  pull_request:

permissions: {}

concurrency:
  group: rakko-update-${{ github.event.pull_request.number }}
  cancel-in-progress: true

jobs:
  update:
    name: Update the tool pins
    runs-on: ubuntu-latest

    permissions:
      contents: write

    steps:
      - name: Update the tool pins
        uses: aonyx-ai/rakko-update@<commit> # main
```

The action checks out the branch itself, so the job needs no other step. It
installs the tools that mise pins for the project, and it builds the harness.

The action has no release yet. Replace `<commit>` with the full hash of a
commit on `main`. Keep the comment `# main` after the hash. Renovate moves a
pinned commit only when such a comment gives the branch or the tag that it
follows.

### Permissions

The action pushes with the token of the workflow, so the job needs
`contents: write`. A workflow that Dependabot starts gets a token that can only
read, also when the settings of the repository allow more. The `permissions`
key of the job gives that token write access too.

### Renovate

Renovate does not update a branch that holds the commit of another author. So
extend the preset of this repository in your configuration of Renovate:

```json
{
  "extends": ["github>aonyx-ai/rakko-update"]
}
```

The preset tells Renovate to ignore the commits of the action. Renovate then
continues to rebase its branch. The preset also extends the preset of Rakko,
`github>aonyx-ai/rakko`, which turns off Renovate for the generated file.
Extend one of the two presets, not both.

Dependabot needs no configuration. The message of each commit of the action
holds `[dependabot skip]`. Dependabot then continues to rebase its branch.

## The Credential

The input `token` is the credential that checks out the branch and pushes the
updated tool pins. The default is the token of the workflow.

| Credential                | Setup                         | CI on the commit of the action        |
| ------------------------- | ----------------------------- | ------------------------------------- |
| The token of the workflow | None                          | Waits until a person approves the run |
| The token of a GitHub App | An app, its key, and a secret | Starts without a person               |
| A personal access token   | A token and a secret          | Starts without a person               |

Start with the token of the workflow. GitHub does not run CI on a push with
that token until a person with write access approves the run with one click.
Each rebase of the branch needs another click. A project whose updates merge
themselves when CI passes then waits for that click.

For updates that need no person, use the token of a GitHub App. The app needs
write access to the contents of the repository. Create a token for the app in
the job. Then pass the token to the action:

```yaml
steps:
  - name: Create the token of the app
    id: app-token
    uses: actions/create-github-app-token@<commit> # v2
    with:
      app-id: ${{ vars.RAKKO_UPDATE_APP_ID }}
      private-key: ${{ secrets.RAKKO_UPDATE_PRIVATE_KEY }}

  - name: Update the tool pins
    uses: aonyx-ai/rakko-update@<commit> # main
    with:
      token: ${{ steps.app-token.outputs.token }}
```

A workflow that Dependabot starts can read only the secrets of Dependabot. So
for Dependabot, store the key of the app as a secret of Dependabot as well.

The action runs the harness of the update with a credential that can write to
the repository. A ruleset can require a pull request for each change to the
default branch. Then the credential cannot push to the default branch.

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
[rakko]: https://github.com/aonyx-ai/rakko
