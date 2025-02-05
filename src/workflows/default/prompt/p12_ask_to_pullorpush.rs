use crate::step::{PromptStep, Step, Task::ActionStepTask};
use crate::workflows::default::action::ta16_pull_rebase_push::PullRebasePush;

use std::io::{self, Write};

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
        // Prompt user to pull or push
        print!("Do you want to pull and rebase, or push the changes? (pull/push/none): ");
        io::stdout().flush().unwrap();
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read input");
        let response = response.trim().to_lowercase();

        match response.as_str() {
            "pull" => Step::Task(ActionStepTask(Box::new(PullRebasePush::new("pull")))),
            "push" => Step::Task(ActionStepTask(Box::new(PullRebasePush::new("push")))),
            _ => Step::Stop,
        }
    }
}
