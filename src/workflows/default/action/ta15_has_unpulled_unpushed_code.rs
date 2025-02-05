use crate::step::{PromptStep, Step, Task::ActionStepTask};
use crate::workflows::default::prompt::pa05_ask_to_pull_or_push::AskToPullOrPush;
use std::process::Command;

pub(crate) struct HasUnpushedOrPulledCode {
    name: String,
}

impl PromptStep for HasUnpushedOrPulledCode {
    fn new(name: &str) -> Self {
        HasUnpushedOrPulledCode {
            name: name.to_owned(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Step {
        // Check for unpushed or unpulled code
        let output = Command::new("git")
            .arg("status")
            .output()
            .expect("Failed to execute git status");

        let status_output = String::from_utf8_lossy(&output.stdout);

        if status_output.contains("Your branch is behind") || status_output.contains("Your branch is ahead") {
            Step::Task(ActionStepTask(Box::new(AskToPullOrPush::new("pull_or_push"))))
        } else {
            Step::Stop
        }
    }
}
