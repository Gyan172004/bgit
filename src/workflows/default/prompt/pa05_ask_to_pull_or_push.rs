use crate::step::{PromptStep, Step, Task::ActionStepTask};
use std::io::{self, Write};
use crate::workflows::default::action::ta17_pull::Pull;
use crate::workflows::default::action::ta18_push::Push;

// use super::ta10_pull,ta11_pull;

pub(crate) struct AskToPullOrPush {
    name: String,
}

impl PromptStep for AskToPullOrPush {
    fn new(name: &str) -> Self {
        AskToPullOrPush {
            name: name.to_owned(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Step {
        // Prompt user for pull or push
        print!("Do you want to pull or push changes? (pull/push/none): ");
        io::stdout().flush().unwrap();
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read input");
        let response = response.trim().to_lowercase();

        match response.as_str() {
            "pull" => Step::Task(ActionStepTask(Box::new(Pull::new("pull")))),
            "push" => Step::Task(ActionStepTask(Box::new(Push::new("push")))),
            _ => Step::Stop
            // Step::Stop
        }
    }
}
