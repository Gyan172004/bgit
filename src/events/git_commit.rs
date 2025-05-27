use colored::Colorize;
use git2::{Repository, Signature};
use std::env;
use std::fs;
use std::path::Path;
use gemini_rs;

use crate::bgit_error::{BGitError, BGitErrorWorkflowType, NO_RULE, NO_STEP};
use crate::events::AtomicEvent;
use crate::rules::Rule;

// Implement conversion from git2::Error to Box<BGitError>
impl From<git2::Error> for Box<BGitError> {
    fn from(error: git2::Error) -> Self {
        Box::new(BGitError::new(
            "Git Error",
            &error.to_string(),
            BGitErrorWorkflowType::RawExecutor,
            NO_STEP,
            "git_commit",
            NO_RULE,
        ))
    }
}

// Implement conversion from std::io::Error to Box<BGitError>
impl From<std::io::Error> for Box<BGitError> {
    fn from(error: std::io::Error) -> Self {
        Box::new(BGitError::new(
            "IO Error",
            &error.to_string(),
            BGitErrorWorkflowType::RawExecutor,
            NO_STEP,
            "git_commit",
            NO_RULE,
        ))
    }
}

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
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.raw_execute_async())
    }
}

impl GitCommit {
    pub fn with_message(message: String) -> Self {
        let mut commit = Self::new();
        commit.message = Some(message);
        commit
    }

    async fn generate_ai_commit_message(&self, repo: &Repository) -> Result<String, Box<dyn std::error::Error>> {
        let mut diff_content = String::new();

        let diff = repo.diff_index_to_workdir(None, None)?;
        diff.print(git2::DiffFormat::Patch, |_, _, line| {
            diff_content.push_str(&format!("{}", String::from_utf8_lossy(line.content())));
            true
        })?;

        let prompt = format!(
            "Generate a concise and descriptive git commit message for these changes. \
             Use conventional commit format (type: description). The message should be \
             under 50 characters. Here are the changes:\n\n{}",
            diff_content
        );

        let response = gemini_rs::chat("gemini-2.0-flash")
            .send_message(&prompt)
            .await?;
        // console::log!("AI response: 
        println!("AI response: {}", response);
        Ok("".to_string())
    }

    async fn raw_execute_async(&self) -> Result<bool, Box<BGitError>> {
        let cwd = env::current_dir()?;
        let repo = Repository::discover(&cwd)?;
        let statuses = repo.statuses(None)?;

        let has_staged_changes = statuses.iter().any(|status| 
            status.status().is_index_new() 
            || status.status().is_index_modified() 
            || status.status().is_index_deleted() 
            || status.status().is_index_renamed() 
            || status.status().is_index_typechange()
        );

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
                match self.generate_ai_commit_message(&repo).await {
                    Ok(msg) => {
                        eprintln!("{} AI generated commit message: {}", "ℹ".blue(), msg.bright_blue());
                        msg
                    },
                    Err(e) => {
                        let commit_msg_path = repo.path().join("COMMIT_EDITMSG");
                        if commit_msg_path.exists() {
                            fs::read_to_string(commit_msg_path)?
                        } else {
                            return Err(Box::new(BGitError::new(
                                "No commit message available",
                                &e.to_string(),
                                BGitErrorWorkflowType::RawExecutor,
                                NO_STEP,
                                self.get_name(),
                                NO_RULE,
                            )));
                        }
                    }
                }
            }
        };

        let signature = repo.signature()?;
        let tree_id = repo.index()?.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        let parent_commit = match repo.head() {
            Ok(head) => Some(head.peel_to_commit()?),
            Err(_) => None,
        };

        let parents = parent_commit.as_ref().map_or(vec![], |commit| vec![commit]);

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &commit_message,
            &tree,
            &parents,
        )?;

        eprintln!(
            "{} Created commit with message: {}",
            "✓".green(),
            commit_message.bright_blue()
        );

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    fn setup_test_repo() -> (TempDir, Repository) {
        let temp_dir = TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();
        
        repo.config().unwrap().set_str("user.name", "Test User").unwrap();
        repo.config().unwrap().set_str("user.email", "test@example.com").unwrap();
        
        (temp_dir, repo)
    }

    #[tokio::test]
    async fn test_commit_with_staged_changes() {
        let (temp_dir, repo) = setup_test_repo();
        
        let test_file_path = temp_dir.path().join("test.txt");
        let mut file = File::create(&test_file_path).unwrap();
        writeln!(file, "Test content").unwrap();
        
        let mut index = repo.index().unwrap();
        index.add_path(Path::new("test.txt")).unwrap();
        index.write().unwrap();
        
        let commit = GitCommit::with_message(String::from("Test commit"));
        assert!(commit.raw_execute().is_ok());
    }

    #[tokio::test]
    async fn test_commit_without_staged_changes() {
        let (temp_dir, _) = setup_test_repo();
        env::set_current_dir(temp_dir.path()).unwrap();
        
        let commit = GitCommit::new();
        assert!(commit.raw_execute().is_err());
    }
}