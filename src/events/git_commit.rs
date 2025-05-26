use colored::Colorize;
use git2::Repository;
use std::env;
use std::fs;

use crate::bgit_error::{BGitError, BGitErrorWorkflowType, NO_RULE, NO_STEP};
use crate::events::AtomicEvent;
use crate::rules::Rule;

pub struct GitCommit {
    name: String,
    action_description: String,
    pre_check_rules: Vec<Box<dyn Rule + Send + Sync>>,
    message: Option<String>,
}

impl AtomicEvent for GitCommit {
    fn new() -> Self {
        GitCommit {
            name: String::from("git_commit"),
            action_description: String::from("Create a new commit with staged changes"),
            pre_check_rules: Vec::new(),
            message: None,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_action_description(&self) -> &str {
        &self.action_description
    }

    fn add_pre_check_rule(&mut self, rule: Box<dyn Rule + Send + Sync>) {
        self.pre_check_rules.push(rule);
    }

    fn get_pre_check_rule(&self) -> &Vec<Box<dyn Rule + Send + Sync>> {
        &self.pre_check_rules
    }

    fn raw_execute(&self) -> Result<bool, Box<BGitError>> {
        let cwd = env::current_dir().map_err(|e| {
            Box::new(BGitError::new(
                "Failed to get current directory",
                &e.to_string(),
                BGitErrorWorkflowType::RawExecutor,
                NO_STEP,
                self.get_name(),
                NO_RULE,
            ))
        })?;

        let repo = Repository::discover(&cwd).map_err(|e| {
            Box::new(BGitError::new(
                "Failed to open repository",
                &e.to_string(),
                BGitErrorWorkflowType::RawExecutor,
                NO_STEP,
                self.get_name(),
                NO_RULE,
            ))
        })?;

        let statuses = repo.statuses(None).map_err(|e| {
            Box::new(BGitError::new(
                "Failed to get repository status",
                &e.to_string(),
                BGitErrorWorkflowType::RawExecutor,
                NO_STEP,
                self.get_name(),
                NO_RULE,
            ))
        })?;

        let has_staged_changes = statuses.iter().any(|status| status.status().is_index_new() 
            || status.status().is_index_modified() 
            || status.status().is_index_deleted() 
            || status.status().is_index_renamed() 
            || status.status().is_index_typechange());

        if !has_staged_changes {
            return Err(Box::new(BGitError::new(
                "No staged changes",
                "There are no changes staged for commit",
                BGitErrorWorkflowType::RawExecutor,
                NO_STEP,
                self.get_name(),
                NO_RULE,
            )));
        }

        let commit_message = match &self.message {
            Some(msg) => msg.clone(),
            None => {
                let commit_msg_path = repo.path().join("COMMIT_EDITMSG");
                if commit_msg_path.exists() {
                    fs::read_to_string(commit_msg_path).map_err(|e| {
                        Box::new(BGitError::new(
                            "Failed to read commit message",
                            &e.to_string(),
                            BGitErrorWorkflowType::RawExecutor,
                            NO_STEP,
                            self.get_name(),
                            NO_RULE,
                        ))
                    })?
                } else {
                    return Err(Box::new(BGitError::new(
                        "No commit message provided",
                        "Please provide a commit message",
                        BGitErrorWorkflowType::RawExecutor,
                        NO_STEP,
                        self.get_name(),
                        NO_RULE,
                    )));
                }
            }
        };

        let signature = repo.signature().map_err(|e| {
            Box::new(BGitError::new(
                "Failed to get signature",
                &e.to_string(),
                BGitErrorWorkflowType::RawExecutor,
                NO_STEP,
                self.get_name(),
                NO_RULE,
            ))
        })?;

        let tree_id = repo.index().unwrap().write_tree().map_err(|e| {
            Box::new(BGitError::new(
                "Failed to write tree",
                &e.to_string(),
                BGitErrorWorkflowType::RawExecutor,
                NO_STEP,
                self.get_name(),
                NO_RULE,
            ))
        })?;

        let tree = repo.find_tree(tree_id).map_err(|e| {
            Box::new(BGitError::new(
                "Failed to find tree",
                &e.to_string(),
                BGitErrorWorkflowType::RawExecutor,
                NO_STEP,
                self.get_name(),
                NO_RULE,
            ))
        })?;

        let parents = if let Ok(head) = repo.head() {
            if let Ok(commit) = head.peel_to_commit() {
                vec![commit]
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &commit_message,
            &tree,
            parents.iter().collect::<Vec<_>>().as_slice(),
        )
        .map_err(|e| {
            Box::new(BGitError::new(
                "Failed to create commit",
                &e.to_string(),
                BGitErrorWorkflowType::RawExecutor,
                NO_STEP,
                self.get_name(),
                NO_RULE,
            ))
        })?;

        eprintln!(
            "{} Created commit with message: {}",
            "✓".green(),
            commit_message.bright_blue()
        );

        Ok(true)
    }
}

impl GitCommit {
    pub fn with_message(message: String) -> Self {
        let mut commit = Self::new();
        commit.message = Some(message);
        commit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use tempfile::TempDir;

    fn setup_test_repo() -> (TempDir, Repository) {
        let temp_dir = TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();
        
        repo.config().unwrap().set_str("user.name", "Test User").unwrap();
        repo.config().unwrap().set_str("user.email", "test@example.com").unwrap();
        
        (temp_dir, repo)
    }

    #[test]
    fn test_commit_with_staged_changes() {
        let (temp_dir, repo) = setup_test_repo();
        
        let test_file_path = temp_dir.path().join("test.txt");
        let mut file = File::create(&test_file_path).unwrap();
        writeln!(file, "Test content").unwrap();
        
        let mut index = repo.index().unwrap();
        index.add_path(Path::new("test.txt")).unwrap();
        index.write().unwrap();
        
        let commit = GitCommit::with_message(String::from("Test commit"));
        assert!(commit.raw_execute().is_ok());
        
        let head_commit = repo.head().unwrap().peel_to_commit().unwrap();
        assert_eq!(head_commit.message().unwrap(), "Test commit");
    }

    #[test]
    fn test_commit_without_staged_changes() {
        let (temp_dir, _) = setup_test_repo();
        env::set_current_dir(temp_dir.path()).unwrap();
        
        let commit = GitCommit::with_message(String::from("Test commit"));
        assert!(commit.raw_execute().is_err());
    }
}