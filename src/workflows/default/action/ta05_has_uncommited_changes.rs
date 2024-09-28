use crate::{
    bgit_error::BGitError,
    common_store::workflow_store::{TASK_COMMIT_CHANGES, TASK_PUSH_CHANGES},
    step::{ActionStep, Step, Task::ActionStepTask},
};
use git2::{Repository, StatusOptions};
use std::env;

// Define some constants for error codes, event codes, and rule codes
const ERROR_CODE_REPO_OPEN: u32 = 1001;
const ERROR_CODE_REPO_STATUS: u32 = 1002;

const EVENT_CODE_UNCOMMITTED_CHECK: u32 = 2002;

const RULE_CODE_STATUS_CHECK: u32 = 3002;

pub(crate) struct HasUncommittedChanges {
    name: String,
}

impl ActionStep for HasUncommittedChanges {
    fn new(name: &str) -> Self
    where
        Self: Sized,
    {
        HasUncommittedChanges {
            name: name.to_owned(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Result<Step, Box<BGitError>> {
        let cwd = env::current_dir().expect("Failed to get current directory");
        let repo = match Repository::discover(cwd) {
            Ok(repo) => repo,
            Err(e) => {
                return Err(Box::new(BGitError::new(
                    "Failed to open Git repository",
                    &format!("Git2 Error: {}", e),
                    "HasUncommittedChanges",
                    ERROR_CODE_REPO_OPEN,
                    EVENT_CODE_UNCOMMITTED_CHECK,
                    RULE_CODE_STATUS_CHECK,
                )));
            }
        };

        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(false);
        let statuses = match repo.statuses(Some(&mut status_opts)) {
            Ok(statuses) => statuses,
            Err(e) => {
                return Err(Box::new(BGitError::new(
                    "Failed to get repository status",
                    &format!("Git2 Error: {}", e),
                    "HasUncommittedChanges",
                    ERROR_CODE_REPO_STATUS,
                    EVENT_CODE_UNCOMMITTED_CHECK,
                    RULE_CODE_STATUS_CHECK,
                )));
            }
        };

        if statuses.is_empty() {
            println!("No uncommitted changes detected.");
            Ok(Step::Task(ActionStepTask(Box::new(
                TASK_PUSH_CHANGES.copy_struct(),
            ))))
        } else {
            println!("Uncommitted changes detected.");
            Ok(Step::Task(ActionStepTask(Box::new(
                TASK_COMMIT_CHANGES.copy_struct(),
            ))))
        }
    }
}
