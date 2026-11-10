//! The maintenance commands of this repository
//!
//! This binary is the harness of this repository: the one place that states
//! which maintenance actions run here. It mounts the bundles that this
//! repository uses, and the command line that it builds turns each of them
//! into a command. A bundle carries a set of actions that projects adopt
//! together, so the harness names the bundle instead of each action in it.
//!
//! It also mounts commands, for a maintenance activity that no single action
//! describes. A command comes from a crate, as an action does, and this file
//! names it. When a command runs actions, this file names those actions as
//! well and gives them to the command.
//!
//! Run it with `mise run rakko`, or with `rakko` where the environment
//! supplies the shortcut.

use rakko_action::ErasedAction;
use rakko_cli::ErasedCommand;
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
/// The call ends the process itself, so a harness stays a `main` that names
/// what the repository mounts and returns nothing.
///
/// The only Rust in the template is this harness, which is a binary, so it
/// mounts the binary bundle beside the baseline. A project that grows out of
/// the template and publishes a library swaps it for the library bundle,
/// which contains everything that the binary bundle carries. A harness stops
/// when two bundles give it two actions of one name, so it never mounts both.
fn main() {
    let pre_commit = PreCommit::new(actions_that_write().into_iter().chain(actions_that_read()));

    rakko_cli::builder()
        .mount(rakko_baseline::bundle())
        .mount(rakko_rust_binary::bundle())
        .mount_commands([Box::new(pre_commit) as Box<dyn ErasedCommand>])
        .run();
}

/// Returns the actions that write to the tree before a commit, in their order
///
/// A formatter that a project adds can overlap with these in the files that
/// it writes, and then the last formatter that writes a file decides its
/// content.
fn actions_that_write() -> Vec<Box<dyn ErasedAction>> {
    vec![
        Box::new(FormatJson),
        Box::new(FormatMarkdown),
        Box::new(FormatYaml),
        Box::new(FormatToml),
        Box::new(FormatRust),
    ]
}

/// Returns the actions that only read the tree before a commit, in their order
///
/// The list is shorter than the list that the harness mounts. An action such
/// as `check-latest-deps`, which resolves the dependencies of the project
/// again, belongs in CI and not in front of every commit.
fn actions_that_read() -> Vec<Box<dyn ErasedAction>> {
    vec![
        Box::new(LintGitHubActions),
        Box::new(LintMarkdown),
        Box::new(LintRust),
        Box::new(LintToml),
        Box::new(LintYaml),
        Box::new(TestRust),
    ]
}
