use crate::{
    bgit_error::BGitError,
    common_store::{event_store::EVENT_GIT_ADD, rules_store::RULE_IS_GIT_INSTALLED_LOCALLY},
    events::AtomicEvent,
    rules::Rule,
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

            let mut git_add_event = EVENT_GIT_ADD.copy_struct();
            git_add_event.add_pre_check_rule(Box::new(RULE_IS_GIT_INSTALLED_LOCALLY.copy_struct()));
            git_add_event.execute()?;

            if has_stash {
                Ok(Step::Stop)
            } else {
                Ok(Step::Stop)
            }
        } else {
            Ok(Step::Stop)
        }
    }
}
