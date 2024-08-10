use std::sync::LazyLock;

use crate::events::{git_add::GitAdd, AtomicEvent};

pub(crate) static EVENT_GIT_ADD: LazyLock<GitAdd> =
    LazyLock::new(|| GitAdd::new("git_add", "Add files to staging area"));
