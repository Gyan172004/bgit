use super::AtomicEvent;
use crate::bgit_error::{BGitError, BGitErrorWorkflowType, NO_RULE, NO_STEP};
use crate::rules::Rule;
use git2::Repository;
use std::env;
use std::path::Path;

pub struct GitClone {
    pub pre_check_rules: Vec<Box<dyn Rule + Send + Sync>>,
    pub url: String,
}

impl AtomicEvent for GitClone {
    fn new() -> Self
    where
        Self: Sized,
    {
        GitClone {
            pre_check_rules: vec![],
            url: String::new(),
        }
    }

    fn get_name(&self) -> &str {
        "git_clone"
    }

    fn get_action_description(&self) -> &str {
        "Clone a Git repository"
    }

    fn add_pre_check_rule(&mut self, rule: Box<dyn Rule + Send + Sync>) {
        self.pre_check_rules.push(rule);
    }

    fn get_pre_check_rule(&self) -> &Vec<Box<dyn Rule + Send + Sync>> {
        &self.pre_check_rules
    }

    fn raw_execute(&self) -> Result<bool, Box<BGitError>> {
        // Check if URL is set
        if self.url.is_empty() {
            return Err(Box::new(BGitError::new(
                "BGitError",
                "Repository URL is not set",
                BGitErrorWorkflowType::AtomicEvent,
                NO_STEP,
                self.get_name(),
                NO_RULE,
            )));
        }
        let url = &self.url;
        let repo_name = match url.split("/").last() {
            Some(repo_name) => repo_name,
            None => {
                return Err(Box::new(BGitError::new(
                    "BGitError",
                    "Failed to get repository name from URL",
                    BGitErrorWorkflowType::AtomicEvent,
                    NO_STEP,
                    self.get_name(),
                    NO_RULE,
                )));
            }
        };
        // Clone repository from the provided URL to the current directory
        Repository::clone(&self.url, Path::new(repo_name)).map_err(|e| {
            Box::new(BGitError::new(
                "BGitError",
                &format!("Failed to clone repository: {}", e),
                BGitErrorWorkflowType::AtomicEvent,
                NO_STEP,
                self.get_name(),
                NO_RULE,
            ))
        })?;

        self.update_cwd_path()?;

        Ok(true)
    }
}

impl GitClone {
    pub fn set_url(&mut self, url: &str) -> &mut Self {
        self.url = url.to_owned();
        self
    }

    fn update_cwd_path(&self) -> Result<(), Box<BGitError>> {
        let repo_name = match self.url.split("/").last() {
            Some(repo_name) => repo_name,
            None => {
                return Err(Box::new(BGitError::new(
                    "BGitError",
                    "Failed to get repository name from URL",
                    BGitErrorWorkflowType::AtomicEvent,
                    NO_STEP,
                    self.get_name(),
                    NO_RULE,
                )));
            }
        };

        match env::set_current_dir(repo_name) {
            Ok(_) => Ok(()),
            Err(_) => Err(Box::new(BGitError::new(
                "Failed to update current working directory path",
                "update_cwd_path",
                BGitErrorWorkflowType::PromptStep,
                NO_STEP,
                self.get_name(),
                NO_RULE,
            ))),
        }
    }
}
