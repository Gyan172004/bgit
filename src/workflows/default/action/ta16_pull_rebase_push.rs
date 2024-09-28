use crate::step::{ActionStep, Step};
use std::process::Command;

pub(crate) struct PullRebasePush {
    action: String,
}

impl PullRebasePush {
    pub(crate) fn new(action: &str) -> Self {
        PullRebasePush {
            action: action.to_owned(),
        }
    }
}

impl ActionStep for PullRebasePush {
    fn execute(&self) -> Step {
        match self.action.as_str() {
            "pull" => {
                // Pull and rebase
                let output = Command::new("git")
                    .args(&["pull", "--rebase"])
                    .output()
                    .expect("Failed to execute git pull");

                if output.status.success() {
                    println!("Pulled and rebased successfully.");
                } else {
                    eprintln!("Failed to pull and rebase.");
                }
                Step::Stop
            }
            "push" => {
                // Push changes
                let output = Command::new("git")
                    .arg("push")
                    .output()
                    .expect("Failed to execute git push");

                if output.status.success() {
                    println!("Pushed changes successfully.");
                } else {
                    eprintln!("Failed to push changes.");
                }
                Step::Stop
            }
            _ => Step::Stop,
        }
    }
}
