use crate::step::{ActionStep, Step};

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
        Step::Stop
    }
}
