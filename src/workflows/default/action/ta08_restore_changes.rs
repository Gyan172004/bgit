use crate::step::{ActionStep, Step, Task::ActionStepTask};
use git2::{Repository, ResetType};
use super::ta05_has_uncommited_changes::HasUncommittedChanges;
use std::env;

pub(crate) struct RestoreChanges {
    name: String,
}

impl RestoreChanges {
    pub(crate) fn new(name: &str) -> Self {
        RestoreChanges {
            name: name.to_owned(),
        }
    }
}

impl ActionStep for RestoreChanges {
    fn new(name: &str) -> Self {
        RestoreChanges::new(name)
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Step {
        let cwd = env::current_dir().expect("Failed to get current directory");
        let repo = match Repository::open(&cwd) {
            Ok(repo) => repo,
            Err(e) => {
                eprintln!("Failed to open Git repository: {}", e);
                return Step::Stop;
            }
        };

        // Get the HEAD reference and peel it to a commit
        let head = match repo.head() {
            Ok(reference) => reference,
            Err(e) => {
                eprintln!("Failed to get HEAD reference: {}", e);
                return Step::Stop;
            }
        };
        
        let commit = match head.peel_to_commit() {
            Ok(commit) => commit,
            Err(e) => {
                eprintln!("Failed to peel HEAD to commit: {}", e);
                return Step::Stop;
            }
        };

        // Reset to the commit (peel to object), which restores unstaged changes
        match repo.reset(&commit.as_object(), ResetType::Mixed, None) {
            Ok(_) => {
                println!("Unstaged changes have been restored.");
                Step::Task(ActionStepTask(Box::new(HasUncommittedChanges::new("check for uncommitted changes"))))
            }
            Err(e) => {
                eprintln!("Failed to restore unstaged changes: {}", e);
                Step::Stop
            }
        }
    }
}
