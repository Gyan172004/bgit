use crate::step::{PromptStep, Step, Task::ActionStepTask};
use crate::workflows::default::action::ta09_commit::Commit;
use crate::workflows::default::action::ta14_make_new_branch_and_move_changes_feature::MakeNewBranchAndMoveChanges;
use std::io::{self, Write};

pub(crate) struct AskSameFeature {
    name: String,
}

impl PromptStep for AskSameFeature {
    fn new(name: &str) -> Self {
        AskSameFeature {
            name: name.to_owned(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Step {
        // Prompt user to check if they are working on the same feature
        print!("Are you working on the same feature? (y/n): ");
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
