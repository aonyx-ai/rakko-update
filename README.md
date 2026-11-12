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

### The Lockfile

When the project has a mise lockfile, the action runs `mise install --locked`
before `doctor --fix`. This command installs every tool that mise pins for the
project, and not only the tools that `doctor --fix` uses.

The runner is the machine that runs the job. Its platform identifies the
operating system and processor architecture, such as `linux-x64`. Every tool
needs a version recorded in the lockfile. For a tool whose download URL mise
records in the lockfile, such as jq, the lockfile also needs a URL and checksum
for the platform of the runner. If the version, the URL, or the checksum is
missing, the install fails before `doctor --fix` runs.

Set `lockfile_platforms` in the `mise.toml` of the project to list the platforms
of the machines that run the tools. The workflow above uses `ubuntu-latest`,
which needs `linux-x64`. This example also covers contributors who use
`macos-arm64`:

```toml
[settings]
lockfile = true
lockfile_platforms = ["linux-x64", "macos-arm64"]
```

When `lockfile_platforms` is set, `mise lock` writes entries for the listed
platforms and for the platform of its machine. It also removes the entries for
other platforms. So when the list holds the platform of the runner, `mise lock`
on another machine keeps the entries for the runner. If the setting is unset,
mise preserves the platforms already in the lockfile. The [mise lockfile
documentation][mise-lockfile] describes the entries of each platform.

After you change the list, run `mise lock` from the project directory. Then
commit the changed configuration and the lockfile.

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
updated tool pins. The default is the token of the workflow, `GITHUB_TOKEN`.

| Credential                | Setup                         | Pull Request Workflows                  |
| ------------------------- | ----------------------------- | --------------------------------------- |
| The token of the workflow | None                          | Waits for one approval per workflow run |
| The token of a GitHub App | An app, its key, and a secret | Starts without a person                 |
| A personal access token   | A token and a secret          | Starts without a person                 |

Start with the token of the workflow. A push with that token causes a
`pull_request` event of the type `synchronize`. For workflows that run on this
event, GitHub [requires approval][github-token-workflows] from a person with
write access. Each workflow run needs its own approval. Three such workflows
therefore need three approvals for each commit of the action. A push with this
token starts no workflow that runs only on `push`.

When the bot rebases its branch, it removes the commit of the action. The
action commits again, and the new workflow runs need approval again. A project
that merges updates when CI passes then waits for these approvals.

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

[github-token-workflows]: https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/trigger-a-workflow#triggering-a-workflow-from-a-workflow
[mise]: https://mise.jdx.dev
[mise-activate]: https://mise.jdx.dev/getting-started.html#activate-mise
[mise-lockfile]: https://mise.jdx.dev/dev-tools/mise-lock.html#preparing-platform-entries
[rakko]: https://github.com/aonyx-ai/rakko
