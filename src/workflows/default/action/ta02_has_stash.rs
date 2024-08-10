use crate::{
    bgit_error::BGitError,
    events::{git_add::GitAdd, AtomicEvent},
    rules::{a01_git_install::IsGitInstalledLocally, Rule, RuleLevel},
    step::{ActionStep, Step},
};
use git2::Repository;
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

    fn execute(&self) -> Result<Step, Box<BGitError>> {
        let cwd = env::current_dir().expect("Failed to get current directory");
        if Repository::discover(&cwd).is_ok() {
            let mut repo = Repository::discover(cwd).unwrap();
            let mut has_stash: bool = false;

            let _ = repo
                .stash_foreach(|_, _, _| {
                    has_stash = true;
                    false
                })
                .is_ok();

            let rule = IsGitInstalledLocally::new(
                "IsGitInstalledLocally",
                "Check if Git is installed",
                RuleLevel::Error,
            );
            let mut git_add_event = GitAdd::new("git_add", "Add files to staging area");
            git_add_event.add_pre_check_rule(Box::new(rule));
            git_add_event.execute()?;

            if has_stash {
                println!("You have stash(es) in your repository");
                Ok(Step::Stop)
            } else {
                println!("You don't have any stash in your repository");
                Ok(Step::Stop)
            }
        } else {
            Ok(Step::Stop)
        }
    }
}
