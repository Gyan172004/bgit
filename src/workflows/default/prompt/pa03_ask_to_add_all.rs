use crate::step::{PromptStep, Step, ActionStep, Task::{ActionStepTask , PromptStepTask}};
use std::io::{self, Write};
use crate::workflows::default::action::ta06_add_all::AddAll;
use crate::workflows::default::action::ta07_add_manual::AddManual;

pub(crate) struct AskToAddAll {
    name: String,
}

impl PromptStep for AskToAddAll {
    fn new(name: &str) -> Self {
        AskToAddAll {
            name: name.to_owned(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Step {
        print!("Do you want to add all files? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "y" => Step::Task(ActionStepTask(Box::new(AddAll::new("add all")))),
            "n" => Step::Task(ActionStepTask(Box::new(AddManual::new("add manual")))),
            _ => {
                println!("Invalid input. Please enter 'y' or 'n'.");
                Step::Task(PromptStepTask(Box::new(AskToAddAll::new("ask to add all"))))
            }
        }
    }
}
