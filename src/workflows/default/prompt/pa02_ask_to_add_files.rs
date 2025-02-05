use crate::step::{PromptStep, Step, Task::{PromptStepTask, ActionStepTask}};
use std::io::{self, Write};
use super::pa03_ask_to_add_all::AskToAddAll;
use super::pa07_ask_restore_changes::RestoreChanges;
// use crate::workflows::default::action::ta08_restore_changes::RestoreChanges;

pub(crate) struct AskToAdd {
    name: String,
}

impl PromptStep for AskToAdd {
    fn new(name: &str) -> Self {
        AskToAdd {
            name: name.to_owned(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Step {
        print!("Do you want to add unstaged files? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "y" => Step::Task(PromptStepTask(Box::new(AskToAddAll::new("ask to add all")))),
            "n" => Step::Task(ActionStepTask(Box::new(RestoreChanges::new("restore changes")))),
            _ => {
                println!("Invalid input. Please enter 'y' or 'n'.");
                Step::Task(PromptStepTask(Box::new(AskToAdd::new("ask to add"))))
            }
        }
    }
}
