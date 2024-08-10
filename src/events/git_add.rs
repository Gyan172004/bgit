use crate::{bgit_error::BGitError, rules::Rule};

use super::AtomicEvent;

pub(crate) struct GitAdd {
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

    fn add_pre_check_rule(&mut self, rule: Box<dyn Rule>) {
        self.pre_check_rules.push(rule);
    }

    fn get_pre_check_rule(&self) -> &Vec<Box<dyn Rule>> {
        &self.pre_check_rules
    }

    fn raw_execute(&self) -> Result<bool, Box<BGitError>> {
        println!("Executing raw_execute for GitAdd");
        Ok(true)
    }
}
