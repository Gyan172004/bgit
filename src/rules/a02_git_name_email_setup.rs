use crate::bgit_error::BGitError;
use crate::rules::{Rule, RuleLevel, RuleOutput};
use std::process::Command;

pub(crate) struct GitNameEmailSetupRule {
    name: String,
    description: String,
    level: RuleLevel,
}

impl Rule for GitNameEmailSetupRule {
    fn new() -> Self {
        GitNameEmailSetupRule {
            name: String::from("RULE_git-name-email-setup"),
            description: String::from("Verify that both user.name and user.email are set in Git configuration before allowing commits"),
            level: RuleLevel::Error,
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
        // Check git user.name
        let name_output = Command::new("git")
            .args(["config", "--get", "user.name"])
            .output()
            .map_err(|e| {
                Box::new(BGitError::new(
                    "Failed to execute git command",
                    &e.to_string(),
                    crate::bgit_error::BGitErrorWorkflowType::Rules,
                    crate::bgit_error::NO_STEP,
                    crate::bgit_error::NO_EVENT,
                    self.get_name(),
                ))
            })?;

        // Check git user.email
        let email_output = Command::new("git")
            .args(["config", "--get", "user.email"])
            .output()
            .map_err(|e| {
                Box::new(BGitError::new(
                    "Failed to execute git command",
                    &e.to_string(),
                    crate::bgit_error::BGitErrorWorkflowType::Rules,
                    crate::bgit_error::NO_STEP,
                    crate::bgit_error::NO_EVENT,
                    self.get_name(),
                ))
            })?;

        let name = String::from_utf8_lossy(&name_output.stdout)
            .trim()
            .to_string();
        let email = String::from_utf8_lossy(&email_output.stdout)
            .trim()
            .to_string();

        if name.is_empty() || email.is_empty() {
            let error_msg = format!(
                "Git user.name and/or user.email is not configured.\n\
                Run:\n\
                  git config --global user.name \"Your Name\"\n\
                  git config --global user.email \"you@example.com\""
            );
            Ok(RuleOutput::Exception(error_msg))
        } else {
            Ok(RuleOutput::Success)
        }
    }

    fn try_fix(&self) -> Result<bool, Box<BGitError>> {
        // Cannot auto-fix as it requires user input
        // Return false to indicate manual intervention is needed
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_creation() {
        let rule = GitNameEmailSetupRule::new();
        assert_eq!(rule.get_name(), "RULE_git-name-email-setup");
        assert_eq!(rule.get_level(), RuleLevel::Error);
    }
}
