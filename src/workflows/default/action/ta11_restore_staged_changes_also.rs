use crate::step::{ActionStep, Step, PromptStep , Task::ActionStepTask};
use git2::{Repository, ResetType};
use crate::workflows::default::action::ta15_has_unpulled_unpushed_code::HasUnpushedOrPulledCode;
use std::env;

pub(crate) struct RestoreStagedChangesAlso {
    name: String,
}

impl RestoreStagedChangesAlso {
    pub(crate) fn new(name: &str) -> Self {
        RestoreStagedChangesAlso {
            name: name.to_owned(),
        }
    }
}

impl ActionStep for RestoreStagedChangesAlso {
    fn new(name: &str) -> Self {
        RestoreStagedChangesAlso::new(name)
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

        // Reset to the commit (peel to object), which restores staged changes
        match repo.reset(&commit.as_object(), ResetType::Hard, None) {
            Ok(_) => {
                println!("Staged changes have been restored.");
                Step::Task(ActionStepTask(Box::new(HasUnpushedOrPulledCode::new("check for unpushed or unpulled code"))))
            }
            Err(e) => {
                eprintln!("Failed to restore staged changes: {}", e);
                Step::Stop
            }
        }
    }
}
