use crate::step::{PromptStep, Step};
use dialoguer::{theme::ColorfulTheme, Confirm};

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
        let confirmation = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to initialize git repository?")
            .default(false)
            .show_default(true)
            .wait_for_newline(true)
            .interact()
            .unwrap();

        println!("Confirmation: {}", confirmation);

        Step::Stop
    }
}
