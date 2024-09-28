use crate::step::{ActionStep, Step, Task::ActionStepTask};
use std::env;
use crate::workflows::default::action::ta04_has_unstaged_files::HasUnstagedFiles;
use crate::workflows::default::action::ta10_pop_stash::PopStash;
use std::io::{self, Write};

pub(crate) struct AskPopStash {
    name: String,
}

impl AskPopStash {
    pub(crate) fn new(name: &str) -> Self {
        AskPopStash {
            name: name.to_owned(),
        }
    }
}

impl ActionStep for AskPopStash {
    fn new(name: &str) -> Self {
        AskPopStash::new(name)
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Step {
        // Prompt user for action
        print!("Do you want to pop the stash? (y/n): ");
        io::stdout().flush().unwrap();
        
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read input");
        let response = response.trim().to_lowercase();
        
        // Decide based on user input
        match response.as_str() {
            "y" | "yes" => {
                println!("Proceeding to pop stash...");
                Step::Task(ActionStepTask(Box::new(PopStash::new("pop stash"))))
            }
            "n" | "no" => {
                println!("Proceeding to check for unstaged files...");
                Step::Task(ActionStepTask(Box::new(HasUnstagedFiles::new("check unstaged files"))))
            }
            _ => {
                eprintln!("Invalid input. Please enter 'y' or 'n'.");
                Step::Stop
            }
        }
    }
}
