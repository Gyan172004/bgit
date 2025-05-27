use crate::bgit_error::BGitError;
use crate::rules::{Rule, RuleLevel, RuleOutput};
use std::process::Command;

pub(crate) struct BigRepoSizeRule {
    name: String,
    description: String,
    level: RuleLevel,
    size_limit_mib: u64, // Size limit in MiB
}

impl Rule for BigRepoSizeRule {
    fn new() -> Self {
        BigRepoSizeRule {
            name: String::from("RULE_big-repo-size"),
            description: String::from(
                "Warn when a repository's total size exceeds a configurable threshold",
            ),
            level: RuleLevel::Warning,
            size_limit_mib: 100, // Default 100 MiB as specified in the docs
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
        let repo_size = self.get_repo_size()?;

        if repo_size > self.size_limit_mib {
            let warning_msg = format!(
                "Repository size ({} MiB) exceeds configured limit of {} MiB.\n\
                Consider using Git LFS or removing large files using git filter-repo.\n\
                Suggested fixes:\n\
                1. Move large assets to Git LFS:\n\
                   git lfs install\n\
                   git lfs track \"*.bin\"\n\
                   git add .gitattributes\n\
                2. Remove large files from history using git filter-repo\n\
                3. Increase size limit if this is intentional",
                repo_size, self.size_limit_mib
            );
            Ok(RuleOutput::Exception(warning_msg))
        } else {
            Ok(RuleOutput::Success)
        }
    }

    fn try_fix(&self) -> Result<bool, Box<BGitError>> {
        // Cannot auto-fix as it requires user intervention
        Ok(false)
    }
}

impl BigRepoSizeRule {
    fn get_repo_size(&self) -> Result<u64, Box<BGitError>> {
        let output = Command::new("git")
            .args(["count-objects", "-vH"])
            .output()
            .map_err(|e| {
                Box::new(BGitError::new(
                    "Failed to execute git count-objects command",
                    &e.to_string(),
                    crate::bgit_error::BGitErrorWorkflowType::Rules,
                    crate::bgit_error::NO_STEP,
                    crate::bgit_error::NO_EVENT,
                    self.get_name(),
                ))
            })?;

        let output_str = String::from_utf8_lossy(&output.stdout);

        // Parse the size-pack line to get repository size
        for line in output_str.lines() {
            if line.starts_with("size-pack:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    // Convert size to MiB, handling different units
                    let size_str = parts[1].to_string();
                    let unit = if parts.len() > 2 { parts[2] } else { "MiB" };

                    let numeric_size = size_str.parse::<f64>().map_err(|e| {
                        Box::new(BGitError::new(
                            "Failed to parse repository size",
                            &e.to_string(),
                            crate::bgit_error::BGitErrorWorkflowType::Rules,
                            crate::bgit_error::NO_STEP,
                            crate::bgit_error::NO_EVENT,
                            self.get_name(),
                        ))
                    })?;

                    // Convert to MiB based on unit
                    let size_in_mib = match unit {
                        "KiB" => numeric_size / 1024.0,
                        "MiB" => numeric_size,
                        "GiB" => numeric_size * 1024.0,
                        _ => numeric_size, // Assume MiB if unit is unknown
                    };

                    return Ok(size_in_mib as u64);
                }
            }
        }

        Err(Box::new(BGitError::new(
            "Failed to parse repository size",
            "Could not find size-pack in git count-objects output",
            crate::bgit_error::BGitErrorWorkflowType::Rules,
            crate::bgit_error::NO_STEP,
            crate::bgit_error::NO_EVENT,
            self.get_name(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_creation() {
        let rule = BigRepoSizeRule::new();
        assert_eq!(rule.get_name(), "RULE_big-repo-size");
        assert_eq!(rule.get_level(), RuleLevel::Warning);
        assert_eq!(rule.size_limit_mib, 100);
    }

    // Additional tests could be added here to test size parsing
    // and threshold checking using mock git commands
}
