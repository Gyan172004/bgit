use super::AtomicEvent;
use crate::bgit_error::{BGitError, BGitErrorWorkflowType, NO_RULE, NO_STEP};
use crate::rules::Rule;
use git2::Repository;
use std::path::Path;

pub struct GitClone {
    pub name: String,
    pub action_description: String,
    pub pre_check_rules: Vec<Box<dyn Rule + Send + Sync>>,
    pub url: String,
}

impl AtomicEvent for GitClone {
    fn new(name: &str, action_description: &str) -> Self
    where
        Self: Sized,
    {
        GitClone {
            name: name.to_owned(),
            action_description: action_description.to_owned(),
            pre_check_rules: vec![],
            url: String::new(),
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

        Ok(true)
    }
}

impl GitClone {
    pub fn set_url(&mut self, url: &str) -> &mut Self {
        self.url = url.to_owned();
        self
    }
}
