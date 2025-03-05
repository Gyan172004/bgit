use crate::{
    bgit_error::BGitError,
    events::{git_init::GitInit, AtomicEvent},
    step::{ActionStep, Step},
};

pub(crate) struct InitGitRepo {
    name: String,
    path: String,
}

impl InitGitRepo {
    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_owned();
    }
}

impl ActionStep for InitGitRepo {
    fn new(name: &str) -> Self {
        InitGitRepo {
            name: name.to_owned(),
            path: ".".to_owned(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Result<Step, Box<BGitError>> {
        let git_init = GitInit::new("git_init", "Initialize git repository").with_path(&self.path);
        git_init.execute()?;
        Ok(Step::Stop)
    }
}
