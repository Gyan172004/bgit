use super::AtomicEvent;
use crate::{
    bgit_error::{BGitError, BGitErrorWorkflowType, NO_RULE, NO_STEP},
    rules::Rule,
};
use git2::{Repository, RepositoryInitOptions};
use std::path::Path;

pub struct GitInit {
    name: String,
    action_description: String,
    pre_check_rules: Vec<Box<dyn Rule + Send + Sync>>,
    path: String, // Add path field
}

impl GitInit {
    // Add a method to set custom path
    pub fn with_path(mut self, path: &str) -> Self {
        self.path = path.to_owned();
        self
    }
}

impl AtomicEvent for GitInit {
    fn new(name: &str, action_description: &str) -> Self
    where
        Self: Sized,
    {
        GitInit {
            name: name.to_owned(),
            action_description: action_description.to_owned(),
            pre_check_rules: vec![],
            path: ".".to_owned(), // Default to current directory
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
        let mut opts = RepositoryInitOptions::new();
        opts.initial_head("main");

        Repository::init_opts(Path::new(&self.path), &opts).map_err(|e| {
            Box::new(BGitError::new(
                "BGitError",
                &format!("Failed to initialize repository at '{}': {}", self.path, e),
                BGitErrorWorkflowType::ActionStep,
                NO_STEP,
                self.get_name(),
                NO_RULE,
            ))
        })?;

        Ok(true)
    }
}
