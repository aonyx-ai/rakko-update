# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [0.1.0] - 2026-11-17

### Added

- Add the action, which runs `mise run rakko -- doctor --fix` in the pull
  request of an update bot and commits the new tool pins to its branch.
- Add the input `token`, the credential that checks out the branch and pushes
  the commit, such as the token of a GitHub App.
- Add the input `bots`, which adds accounts to the bots whose pull requests the
  action updates, next to `renovate[bot]` and `dependabot[bot]`.
- Add the Renovate preset `github>aonyx-ai/rakko-update`, which extends the
  preset of Rakko and lets Renovate keep rebasing after a commit of the action.

[keep a changelog]: https://keepachangelog.com/en/1.1.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
[0.1.0]: https://github.com/aonyx-ai/rakko-update/releases/tag/v0.1.0
