use crate::{
    bgit_error::BGitError,
    step::{ActionStep, Step, Task::ActionStepTask},
    common_store::workflow_store::TASK_HAS_STASH,
};
use git2::{Repository, Error};
use std::env;
use std::path::Path;

pub(crate) struct InitGitRepo {
    name: String,
}

impl ActionStep for InitGitRepo {
    fn new(name: &str) -> Self
    where
        Self: Sized,
    {
        InitGitRepo {
            name: name.to_owned(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Result<Step, Box<BGitError>> {
        let cwd = env::current_dir().expect("Failed to get current directory");
        let path = Path::new(&cwd);

        // Attempt to initialize a new Git repository in the current directory
        match Repository::init(path) {
            Ok(_) => {
                println!("Initialized empty Git repository in {}", cwd.display());
                Ok(Step::Task(ActionStepTask(Box::new(
                    TASK_HAS_STASH.copy_struct(),
                ))))
            }
            Err(e) => {
                eprintln!("Failed to initialize Git repository: {}", e);
                Err(Box::new(BGitError::new(
                    "Failed to initialize Git repository",
                    &format!("Git2 Error: {}", e),
                    "InitGitRepo",
                    0, // Replace with appropriate error code
                    0, // Replace with appropriate event code
                    0, // Replace with appropriate rule code
                )))
            }
        }
    }
}
