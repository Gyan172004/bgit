use crate::step::{PromptStep, Step, Task::ActionStepTask};
use crate::workflows::default::action::ta09_commit::Commit;
use std::io::{self, Write};

pub(crate) struct AskForMessage {
    name: String,
}

impl PromptStep for AskForMessage {
    fn new(name: &str) -> Self {
        AskForMessage {
            name: name.to_owned(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Step {
        // Prompt user for commit message
        print!("Enter commit message: ");
        io::stdout().flush().unwrap();
        let mut message = String::new();
        io::stdin().read_line(&mut message).expect("Failed to read input");
        let message = message.trim();

        Step::Task(ActionStepTask(Box::new(Commit::new(message))))
    }
}
