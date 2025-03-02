use crate::bgit_error::{BGitError, NO_EVENT, NO_STEP};
use crate::rules::{Rule, RuleLevel, RuleOutput};
use std::process::Command;

pub(crate) struct IsGitInstalledLocally {
    name: String,
    description: String,
    level: RuleLevel,
}

impl Rule for IsGitInstalledLocally {
    fn new(name: &str, description: &str, level: RuleLevel) -> Self {
        IsGitInstalledLocally {
            name: name.to_string(),
            description: description.to_string(),
            level,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_description(&self) -> &str {
        &self.description
    }

    fn get_level(&self) -> RuleLevel {
        self.level.clone()
    }

    fn check(&self) -> Result<RuleOutput, Box<BGitError>> {
        let output = Command::new("git").arg("--version").output();
        match output {
            Err(e) => Ok(RuleOutput::Exception(format!(
                "Failed to execute command: {}",
                e
            ))),
            Ok(output_response) => {
                if output_response.status.success() {
                    Ok(RuleOutput::Success)
                } else {
                    Ok(RuleOutput::Exception("Git is not installed".to_string()))
                }
            }
        }
    }

    fn try_fix(&self) -> Result<bool, Box<BGitError>> {
        println!("Executing sudo apt-get install git");
        let output = Command::new("sudo")
            .arg("apt-get")
            .arg("install")
            .arg("git")
            .output();

        match output {
            Err(e) => Err(Box::new(BGitError::new(
                "Failed to execute command",
                &e.to_string(),
                "IsGitInstalledLocally",
                NO_STEP,
                NO_EVENT,
                self.get_name(),
            ))),
            Ok(output_response) => {
                if output_response.status.success() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }
}
