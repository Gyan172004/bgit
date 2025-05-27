use crate::bgit_error::BGitError;
use crate::rules::{Rule, RuleLevel, RuleOutput};
use std::process::Command;

pub(crate) struct FileNotGitIgnoredRule {
    name: String,
    description: String,
    level: RuleLevel,
}

impl Rule for FileNotGitIgnoredRule {
    fn new() -> Self {
        FileNotGitIgnoredRule {
            name: String::from("RULE_file_not_gitignored"),
            description: String::from("Check for files that should be gitignored but are staged"),
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
        // Get staged files
        let staged_files = self.get_staged_files()?;
        let mut violations = Vec::new();

        // Check each staged file against gitignore rules
        for file in staged_files {
            if self.should_be_ignored(&file)? {
                violations.push(file);
            }
        }

        if !violations.is_empty() {
            let error_msg = format!(
                "The following files are staged but should be gitignored according to your .gitignore rules:\n{}\n\
                Please update .gitignore if needed and unstage these files using:\n\
                git reset HEAD <file>",
                violations.join("\n")
            );
            Ok(RuleOutput::Exception(error_msg))
        } else {
            Ok(RuleOutput::Success)
        }
    }

    fn try_fix(&self) -> Result<bool, Box<BGitError>> {
        // Cannot auto-fix as it requires user intervention
        Ok(false)
    }
}

impl FileNotGitIgnoredRule {
    fn get_staged_files(&self) -> Result<Vec<String>, Box<BGitError>> {
        let output = Command::new("git")
            .args(["diff", "--cached", "--name-only"])
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

        let files = String::from_utf8_lossy(&output.stdout)
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();

        Ok(files)
    }

    fn should_be_ignored(&self, file: &str) -> Result<bool, Box<BGitError>> {
        let output = Command::new("git")
            .args(["check-ignore", file])
            .output()
            .map_err(|e| {
                Box::new(BGitError::new(
                    "Failed to execute git check-ignore",
                    &e.to_string(),
                    crate::bgit_error::BGitErrorWorkflowType::Rules,
                    crate::bgit_error::NO_STEP,
                    crate::bgit_error::NO_EVENT,
                    self.get_name(),
                ))
            })?;

        // If the file should be ignored, git check-ignore exits with status 0
        Ok(output.status.success())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    fn setup_test_env() -> Result<(), std::io::Error> {
        // Create .gitignore file
        let mut gitignore = File::create(".gitignore")?;
        writeln!(gitignore, ".env")?;

        // Add and commit .gitignore first
        Command::new("git").args(["add", ".gitignore"]).output()?;
        Command::new("git")
            .args(["commit", "-m", "Add .gitignore"])
            .output()?;

        // Create .env file
        let mut env_file = File::create(".env")?;
        writeln!(env_file, "SECRET_KEY=test123")?;

        // Remove .env from index if tracked
        let _ = Command::new("git")
            .args(["rm", "--cached", ".env"])
            .output();

        // Stage the .env file
        Command::new("git").args(["add", ".env"]).output()?;

        Ok(())
    }

    fn cleanup_test_env() {
        // Clean up test files
        let _ = std::fs::remove_file(".env");
        let _ = Command::new("git")
            .args(["reset", "--hard", "HEAD"])
            .output();
        let _ = Command::new("git").args(["clean", "-fd"]).output();
    }

    #[test]
    fn test_ignored_file_detection() {
        // Setup test environment
        setup_test_env().expect("Failed to setup test environment");

        let rule = FileNotGitIgnoredRule::new();
        match rule.check() {
            Ok(RuleOutput::Exception(_)) => {
                // Test passed - rule detected the violation
                assert!(true);
            }
            Ok(RuleOutput::Success) => {
                panic!("Rule should have detected staged .env file");
            }
            _ => {
                panic!("Unexpected rule output");
            }
        }

        // Cleanup
        cleanup_test_env();
    }
}
