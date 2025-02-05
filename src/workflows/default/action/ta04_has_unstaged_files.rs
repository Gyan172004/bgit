use crate::{
    bgit_error::BGitError,
    common_store::workflow_store::{TASK_ADD_FILES, TASK_COMMIT_CHANGES},
    step::{ActionStep, Step, Task::ActionStepTask},
};
use git2::{Repository, StatusOptions};
use std::env;

pub(crate) struct HasUnstagedFiles {
    name: String,
}

impl ActionStep for HasUnstagedFiles {
    fn new(name: &str) -> Self
    where
        Self: Sized,
    {
        HasUnstagedFiles {
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
                    "HasUnstagedFiles",
                    0, // Replace with appropriate error code
                    0, // Replace with appropriate event code
                    0, // Replace with appropriate rule code
                )));
            }
        };

        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(true).recurse_untracked_dirs(true);
        let statuses = match repo.statuses(Some(&mut status_opts)) {
            Ok(statuses) => statuses,
            Err(e) => {
                return Err(Box::new(BGitError::new(
                    "Failed to get repository status",
                    &format!("Git2 Error: {}", e),
                    "HasUnstagedFiles",
                    0, // Replace with appropriate error code
                    0, // Replace with appropriate event code
                    0, // Replace with appropriate rule code
                )));
            }
        };

        if statuses.is_empty() {
            println!("No unstaged files detected.");
            Ok(Step::Task(ActionStepTask(Box::new(
                TASK_COMMIT_CHANGES.copy_struct(),
            ))))
        } else {
            println!("Unstaged files detected.");
            Ok(Step::Task(ActionStepTask(Box::new(
                TASK_ADD_FILES.copy_struct(),
            ))))
        }
    }
}
