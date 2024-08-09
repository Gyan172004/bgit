use crate::{bgit_error::BGitError, rules::Rule};

use super::AtomicEvent;

struct GitAdd {
    name: String,
    action_description: String,
    pre_check_rules: Vec<Box<dyn Rule>>,
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

    fn execute(&self) -> Result<bool, crate::bgit_error::BGitError> {
        
        for rule in self.get_pre_check_rule().iter() {
            let rule_passed = rule.execute()?; 
            if !rule_passed {
                return Err(
                    BGitError::new(
                        rule.get_name(),
                        rule.get_description(),
                        "blah",
                        "blah"
                    )
                );
            }
        }

        let pre_commit_hook_status = self.pre_execute_hook()?;
        if !pre_commit_hook_status {
            return Ok(false)
        }

        // execute steps here
        
        let post_commit_hook_status = self.post_execute_hook()?;
        if !post_commit_hook_status {
            return Ok(false)
        }
        Ok(true)
    }

    fn check_rules(&self) -> Result<bool, crate::bgit_error::BGitError> {
        Ok(true)
    }

    fn pre_execute_hook(&self) -> Result<bool, crate::bgit_error::BGitError> {
        Ok(true)
    }

    fn post_execute_hook(&self) -> Result<bool, crate::bgit_error::BGitError> {
        Ok(true)
    }

    fn add_pre_check_rule(&mut self, rule: Box<dyn Rule>) {
        self.pre_check_rules.push(rule);
    }

    fn get_pre_check_rule(&self) -> &Vec<Box<dyn Rule>> {
        &self.pre_check_rules
    }
}
