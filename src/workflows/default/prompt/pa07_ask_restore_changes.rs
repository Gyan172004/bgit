use crate::step::{ActionStep, Step, Task::ActionStepTask};
use git2::{Repository, ResetType};
use std::env;
use crate::workflows::default::action::ta05_has_uncommited_changes::HasUncommittedChanges;
use super::pa08_ask_restore_staged_changes_also::AskRestoreStagedChangesAlso;
use std::io::{self, Write};

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

        // Reset to the commit (peel to object)
        match repo.reset(&commit.as_object(), ResetType::Hard, None) {
            Ok(_) => {
                println!("Changes have been restored.");
                
                // Ask if the user wants to restore staged changes as well
                print!("Do you also want to restore staged changes? (y/n): ");
                io::stdout().flush().unwrap();
                
                let mut response = String::new();
                io::stdin().read_line(&mut response).expect("Failed to read input");
                let response = response.trim().to_lowercase();
                
                // Decide based on user input
                match response.as_str() {
                    "y" | "yes" => {
                        println!("Proceeding to ask about restoring staged changes...");
                        Step::Task(ActionStepTask(Box::new(AskRestoreStagedChangesAlso::new("ask to restore staged changes"))))
                    }
                    "n" | "no" => {
                        println!("Proceeding to check for uncommitted changes...");
                        Step::Task(ActionStepTask(Box::new(HasUncommittedChanges::new("check uncommitted changes"))))
                    }
                    _ => {
                        eprintln!("Invalid input. Please enter 'y' or 'n'.");
                        Step::Stop
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to restore changes: {}", e);
                Step::Stop
            }
        }
    }
}
