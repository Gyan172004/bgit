use crate::step::{ActionStep, PromptStep, Step, Task::{ActionStepTask, PromptStepTask}};
use git2::Repository;
use std::env;

use super::ta02_has_stash::HasStash;
use crate::workflows::default::prompt::pa01_ask_to_init_git::AskToInitGit;
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

    fn execute(&self) -> Step {
        let cwd = env::current_dir().expect("Failed to get current directory");
        if Repository::discover(cwd).is_ok() {
           Step::Task(ActionStepTask(Box::new(HasStash::new("has stash")))) 
        }
        else {
            Step::Task(PromptStepTask(Box::new(AskToInitGit::new("ask to init git"))))
        }
    }
}
