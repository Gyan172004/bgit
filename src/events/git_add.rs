use super::AtomicEvent;
use crate::{
    bgit_error::{BGitError, BGitErrorWorkflowType, NO_EVENT, NO_RULE},
    rules::Rule,
};
use git2::{IndexAddOption, Repository};
use std::path::Path;

pub(crate) struct GitAdd {
    name: String,
    action_description: String,
    pre_check_rules: Vec<Box<dyn Rule + Send + Sync>>,
}

impl AtomicEvent for GitAdd {
    fn new(name: &str, action_description: &str) -> Self
    where
        Self: Sized,
    {
        GitAdd {
            name: name.to_owned(),
            action_description: action_description.to_owned(),
            pre_check_rules: vec![],
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
        // Open the repository at the current directory
        let repo = Repository::discover(Path::new(".")).map_err(|e| {
            Box::new(BGitError::new(
                "BGitError",
                &format!("Failed to open repository: {}", e),
                BGitErrorWorkflowType::AtomicEvent,
                NO_EVENT,
                &self.name,
                NO_RULE,
            ))
        })?;

        // Get the repository index
        let mut index = repo.index().map_err(|e| {
            Box::new(BGitError::new(
                "BGitError",
                &format!("Failed to get repository index: {}", e),
                BGitErrorWorkflowType::AtomicEvent,
                NO_EVENT,
                &self.name,
                NO_RULE,
            ))
        })?;

        // Using ["."], which indicates the current directory recursively.
        index
            .add_all(["."], IndexAddOption::DEFAULT, None)
            .map_err(|e| {
                Box::new(BGitError::new(
                    "BGitError",
                    &format!("Failed to add files to index: {}", e),
                    BGitErrorWorkflowType::AtomicEvent,
                    NO_EVENT,
                    &self.name,
                    NO_RULE,
                ))
            })?;

        // Write the index changes to disk
        index.write().map_err(|e| {
            Box::new(BGitError::new(
                "BGitError",
                &format!("Failed to write index: {}", e),
                BGitErrorWorkflowType::AtomicEvent,
                NO_EVENT,
                &self.name,
                NO_RULE,
            ))
        })?;

        Ok(true)
    }
}
