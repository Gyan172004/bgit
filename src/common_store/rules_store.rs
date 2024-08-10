use std::sync::LazyLock;

use crate::rules::{a01_git_install::IsGitInstalledLocally, Rule, RuleLevel};

pub(crate) static RULE_IS_GIT_INSTALLED_LOCALLY: LazyLock<IsGitInstalledLocally> =
    LazyLock::new(|| {
        IsGitInstalledLocally::new(
            "IsGitInstalledLocally",
            "Check if Git is installed",
            RuleLevel::Error,
        )
    });
