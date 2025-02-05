use crate::step::{ActionStep, Step, Task::ActionStepTask};
use std::io::{self, Write};
use crate::workflows::default::action::ta08_restore_changes::RestoreChanges;
use crate::workflows::default::action::ta11_restore_staged_changes_also::RestoreStagedChangesAlso;

pub(crate) struct AskRestoreStagedChangesAlso {
    name: String,
}

impl AskRestoreStagedChangesAlso {
    pub(crate) fn new(name: &str) -> Self {
        AskRestoreStagedChangesAlso {
            name: name.to_owned(),
        }
    }
}

impl ActionStep for AskRestoreStagedChangesAlso {
    fn new(name: &str) -> Self {
        AskRestoreStagedChangesAlso::new(name)
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Step {
        // Prompt user for action
        print!("Do you also want to restore staged changes? (y/n): ");
        io::stdout().flush().unwrap();
        
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read input");
        let response = response.trim().to_lowercase();
        
        // Decide based on user input
        match response.as_str() {
            "y" | "yes" => {
                println!("Proceeding to restore staged changes...");
                Step::Task(ActionStepTask(Box::new(RestoreStagedChangesAlso::new("restore staged changes"))))
            }
            "n" | "no" => {
                println!("Proceeding to restore changes...");
                Step::Task(ActionStepTask(Box::new(RestoreChanges::new("restore changes"))))
            }
            _ => {
                eprintln!("Invalid input. Please enter 'y' or 'n'.");
                Step::Stop
            }
        }
    }
}
