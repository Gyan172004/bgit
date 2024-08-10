use crate::{
    bgit_error::BGitError,
    common_store::worflow_store::{TASK_ASK_TO_INIT_GIT, TASK_HAS_STASH},
    step::{
        ActionStep, PromptStep, Step,
        Task::{ActionStepTask, PromptStepTask},
    },
};
use git2::Repository;
use std::env;

pub(crate) struct IsGitRepo {
    name: String,
}

impl ActionStep for IsGitRepo {
    fn new(name: &str) -> Self
    where
        Self: Sized,
    {
        IsGitRepo {
            name: name.to_owned(),
        }
    }
    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Result<Step, Box<BGitError>> {
        let cwd = env::current_dir().expect("Failed to get current directory");
        if Repository::discover(cwd).is_ok() {
            Ok(Step::Task(ActionStepTask(Box::new(
                TASK_HAS_STASH.copy_struct(),
            ))))
        } else {
            Ok(Step::Task(PromptStepTask(Box::new(
                TASK_ASK_TO_INIT_GIT.copy_struct(),
            ))))
        }
    }
}
