use crate::step::{PromptStep, Step, Task::ActionStepTask};
// use crate::workflows::default::action::ta09_commit::Commit;
use crate::workflows::default::prompt::p11_ask_for_message::AskForMessage;
use crate::workflows::default::action::ta15_has_unpulled_unpushed_code::HasUnpushedOrPulledCode;

use std::io::{self, Write};

pub(crate) struct AskToCommit {
    name: String,
}

impl PromptStep for AskToCommit {
    fn new(name: &str) -> Self {
        AskToCommit {
            name: name.to_owned(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Step {
        // Prompt user for confirmation
        print!("Do you want to commit the changes? (yes/no): ");
        io::stdout().flush().unwrap();
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read input");
        let response = response.trim().to_lowercase();

        match response.as_str() {
            "yes" => Step::Task(ActionStepTask(Box::new(AskForMessage::new("commit_message")))),
            _ => Step::Task(ActionStepTask(Box::new(HasUnpushedOrPulledCode::new("check_code")))),
        }
    }
}
