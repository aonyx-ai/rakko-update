//! The maintenance commands of this repository
//!
//! This binary is the harness of this repository: the one place that states
//! which maintenance actions run here. It mounts the bundles that this
//! repository uses, and the command line that it builds turns each of them
//! into a command. A bundle carries a set of actions that projects adopt
//! together, so the harness names the bundle instead of each action in it.
//!
//! Each action pins the tools that it runs, and `doctor --fix` writes the pins
//! of the mounted actions into `.config/mise/conf.d/rakko.toml`. So a change to
//! what this file mounts can change the tools that mise installs.
//!
//! It also mounts groups, such as pre-commit. A group is a name and a list of
//! actions that the command line runs as one command. This file gives each
//! group its list.
//!
//! Run it with `mise run rakko`, or with `rakko` where the environment
//! supplies the shortcut.

use rakko::Bundle;
use rakko_format_json::FormatJson;
use rakko_format_markdown::FormatMarkdown;
use rakko_format_rust::FormatRust;
use rakko_format_toml::FormatToml;
use rakko_format_yaml::FormatYaml;
use rakko_lint_github_actions::LintGitHubActions;
use rakko_lint_markdown::LintMarkdown;
use rakko_lint_rust::LintRust;
use rakko_lint_toml::LintToml;
use rakko_lint_yaml::LintYaml;
use rakko_pre_commit::PreCommit;
use rakko_test_rust::TestRust;

/// Builds the command line of this repository and runs it
///
/// The only Rust in the template is this harness, which is a binary, so it
/// mounts the binary bundle beside the baseline. A project that grows out of
/// the template and publishes a library swaps it for the library bundle,
/// which contains everything that the binary bundle carries. A harness stops
/// when two bundles give it two actions of one name, so it never mounts both.
fn main() -> std::process::ExitCode {
    let pre_commit = PreCommit::new(actions_that_guard_a_commit());

    rakko_cli::builder()
        .mount(rakko_baseline::bundle())
        .mount(rakko_rust_binary::bundle())
        .mount(pre_commit)
        .run()
}

/// Returns the actions that guard a commit
///
/// The list is shorter than the list that the harness mounts. An action such
/// as `check-latest-deps`, which resolves the dependencies of the project
/// again, belongs in CI and not in front of every commit.
///
/// The pre-commit command runs the actions that write before the actions that
/// read, so the list needs no order between the two. A formatter that a
/// project adds can overlap with these in the files that it writes. Then the
/// last formatter in the list that writes a file decides its content.
fn actions_that_guard_a_commit() -> Bundle {
    Bundle::default()
        .with(FormatJson)
        .with(FormatMarkdown)
        .with(FormatYaml)
        .with(FormatToml)
        .with(FormatRust)
        .with(LintGitHubActions)
        .with(LintMarkdown)
        .with(LintRust)
        .with(LintToml)
        .with(LintYaml)
        .with(TestRust)
}
