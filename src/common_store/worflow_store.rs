use std::sync::LazyLock;

use crate::step::{ActionStep, PromptStep};
use crate::workflows::default::action::ta01_is_git_repo::IsGitRepo;
use crate::workflows::default::action::ta02_has_stash::HasStash;
use crate::workflows::default::prompt::pa01_ask_to_init_git::AskToInitGit;

pub(crate) static TASK_IS_GIT_REPO: LazyLock<IsGitRepo> =
    LazyLock::new(|| IsGitRepo::new("is_git_repo"));

pub(crate) static TASK_HAS_STASH: LazyLock<HasStash> = LazyLock::new(|| HasStash::new("has_stash"));

pub(crate) static TASK_ASK_TO_INIT_GIT: LazyLock<AskToInitGit> =
    LazyLock::new(|| AskToInitGit::new("ask_to_init_git"));
