use crate::step::{ActionStep, PromptStep, Step, Task::ActionStepTask};
use git2::Repository;
use crate::workflows::default::prompt::pa02_ask_to_add_files::AskToAdd;
use std::env;

pub(crate) struct PopStash {
    name: String,
}

impl PopStash {
    pub(crate) fn new(name: &str) -> Self {
        PopStash {
            name: name.to_owned(),
        }
    }
}

impl ActionStep for PopStash {
    fn new(name: &str) -> Self {
        PopStash::new(name)
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Step {
        let cwd = env::current_dir().expect("Failed to get current directory");

        // Open the repository
        match Repository::discover(&cwd) {
            Ok(repo) => {
                // Attempt to pop the stash
                match repo.stash_pop(0, None) {
                    Ok(_) => {
                        println!("Successfully popped the stash.");
                        // Return the next task to ask to add changes
                        Step::Task(ActionStepTask(Box::new(AskToAdd::new("ask to add changes"))))
                    }
                    Err(e) => {
                        eprintln!("Failed to pop stash: {}", e);
                        Step::Stop
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to discover Git repository: {}", e);
                Step::Stop
            }
        }
    }
}
