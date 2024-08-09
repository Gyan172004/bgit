use crate::step::{PromptStep, Step};

pub(crate) struct AskToInitGit {
    name: String,
}

impl PromptStep for AskToInitGit {
    fn new(name: &str) -> Self
    where
        Self: Sized,
    {
        AskToInitGit {
            name: name.to_owned(),
        }
    }
    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Step {
        println!("Do you want to initialize git repository? (y/n)");
        Step::Stop
    }
}
