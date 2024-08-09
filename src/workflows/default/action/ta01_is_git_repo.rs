use crate::step::{ActionStep, Step};
use git2::Repository;
use std::env;
use std::path::Path;

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
        println!("{}", Repository::discover(cwd).is_ok());
        
        Step::Stop
    }
}
