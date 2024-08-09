use crate::step::{ActionStep, Step};
use git2::{Repository, StatusOptions};
use std::env;

pub(crate) struct HasStash {
    name: String,
} 

impl ActionStep for HasStash {
    fn new(name: &str) -> Self
    where
        Self: Sized,
    {
        HasStash {
            name: name.to_owned(),
        }
    }
    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Step {
        let cwd = env::current_dir().expect("Failed to get current directory");
        if Repository::discover(&cwd).is_ok() {
            let mut repo = Repository::discover(cwd).unwrap();
            let mut has_stash: bool = false;

            let _ = repo.stash_foreach(|_, _, _| {
                has_stash = true;
                false
            }).is_ok();
            
            if has_stash {
                println!("You have stash(es) in your repository");
                Step::Stop
            }
            else {
                println!("You don't have any stash in your repository");
                Step::Stop
            }
        }
        else {
            Step::Stop
        }
    }
}
