use crate::step::{ActionStep, Step};
use std::process::Command;

pub(crate) struct Pull {
    action: String,
}

impl Pull {
    pub(crate) fn new(action: &str) -> Self {
        Pull {
            action: action.to_owned(),
        }
    }
}

impl ActionStep for Pull {
    fn execute(&self) -> Step {
        // Perform git pull operation
        let output = Command::new("git")
            .args(&["pull", "--rebase"]) // Adjust args if needed
            .output()
            .expect("Failed to execute git pull");

        if output.status.success() {
            println!("Pulled changes successfully.");
            Step::Stop
        } else {
            eprintln!("Failed to pull changes: {}", String::from_utf8_lossy(&output.stderr));
            Step::Stop
        }
    }
}
