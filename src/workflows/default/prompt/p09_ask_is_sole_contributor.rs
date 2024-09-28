use crate::step::{PromptStep, Step, Task::ActionStepTask};
use crate::workflows::default::action::ta09_commit::Commit;
use crate::workflows::default::action::ta13_make_new_branch_and_move_changes::MakeNewBranchAndMoveChanges;
use std::io::{self, Write};

pub(crate) struct AskIsSoleContributor {
    name: String,
}

impl PromptStep for AskIsSoleContributor {
    fn new(name: &str) -> Self {
        AskIsSoleContributor {
            name: name.to_owned(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Step {
        // Prompt user to check if they are the sole contributor
        print!("Are you the sole contributor to this repository? (y/n): ");
        io::stdout().flush().unwrap();
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read input");
        let response = response.trim().to_lowercase();

        match response.as_str() {
            "y" => {
                Step::Task(ActionStepTask(Box::new(Commit::new("commit changes"))))
            }
            "n" => {
                Step::Task(ActionStepTask(Box::new(MakeNewBranchAndMoveChanges::new("make new branch and move changes"))))
            }
            _ => {
                eprintln!("Invalid input. Please enter 'y' or 'n'.");
                Step::Stop
            }
        }
    }
}
