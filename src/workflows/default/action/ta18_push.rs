use crate::step::{ActionStep, Step};
use std::process::Command;

pub(crate) struct Push {
    action: String,
}

impl Push {
    pub(crate) fn new(action: &str) -> Self {
        Push {
            action: action.to_owned(),
        }
    }
}

impl ActionStep for Push {
    fn execute(&self) -> Step {
        // Perform git push operation
        let output = Command::new("git")
            .arg("push")
            .output()
            .expect("Failed to execute git push");

        if output.status.success() {
            println!("Pushed changes successfully.");
            Step::Stop
        } else {
            eprintln!("Failed to push changes: {}", String::from_utf8_lossy(&output.stderr));
            Step::Stop
        }
    }
}
