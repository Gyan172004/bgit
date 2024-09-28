use crate::{
    bgit_error::BGitError,
    common_store::workflow_store::TASK_COMMIT_CHANGES,
    step::{ActionStep, Step, Task::ActionStepTask},
};
use git2::{Repository, IndexAddOption};
use std::env;

// Define some constants for error codes, event codes, and rule codes
const ERROR_CODE_REPO_OPEN: u32 = 1001;
const ERROR_CODE_ADD_FILES: u32 = 1003;

const EVENT_CODE_ADD_ALL: u32 = 2003;

const RULE_CODE_ADD_FILES: u32 = 3003;

pub(crate) struct AddAllFiles {
    name: String,
}

impl ActionStep for AddAllFiles {
    fn new(name: &str) -> Self
    where
        Self: Sized,
    {
        AddAllFiles {
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
                    "AddAllFiles",
                    ERROR_CODE_REPO_OPEN,
                    EVENT_CODE_ADD_ALL,
                    RULE_CODE_ADD_FILES,
                )));
            }
        };

        let mut index = match repo.index() {
            Ok(index) => index,
            Err(e) => {
                return Err(Box::new(BGitError::new(
                    "Failed to get repository index",
                    &format!("Git2 Error: {}", e),
                    "AddAllFiles",
                    ERROR_CODE_ADD_FILES,
                    EVENT_CODE_ADD_ALL,
                    RULE_CODE_ADD_FILES,
                )));
            }
        };

        if let Err(e) = index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None) {
            return Err(Box::new(BGitError::new(
                "Failed to add files to staging",
                &format!("Git2 Error: {}", e),
                "AddAllFiles",
                ERROR_CODE_ADD_FILES,
                EVENT_CODE_ADD_ALL,
                RULE_CODE_ADD_FILES,
            )));
        }

        if let Err(e) = index.write() {
            return Err(Box::new(BGitError::new(
                "Failed to write index",
                &format!("Git2 Error: {}", e),
                "AddAllFiles",
                ERROR_CODE_ADD_FILES,
                EVENT_CODE_ADD_ALL,
                RULE_CODE_ADD_FILES,
            )));
        }

        println!("All unstaged files have been added to the staging area.");
        Ok(Step::Task(ActionStepTask(Box::new(
            TASK_COMMIT_CHANGES.copy_struct(),
        ))))
    }
}
