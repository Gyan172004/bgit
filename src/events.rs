use crate::{bgit_error::{BGitError, NO_RULE, NO_STEP}, rules::Rule};
pub(crate) mod git_add;
mod git_branch;
mod git_checkout;
mod git_clean;
mod git_clone;
mod git_commit;
mod git_filter_repo;
mod git_init;
mod git_pull;
mod git_push;
mod git_restore;
mod git_status;

/// Sample struct
/// struct GitAdd {
///     name: String,
///     action_description: String,
///     pre_check_rules: Vec<Box<dyn Rule>>
/// }
/// List of various Git Events to be called with git2-rs library
pub(crate) trait AtomicEvent {
    fn new(name: &str, action_description: &str) -> Self
    where
        Self: Sized;
    fn get_name(&self) -> &str;
    fn get_action_description(&self) -> &str;
    fn add_pre_check_rule(&mut self, rule: Box<dyn Rule>);
    fn get_pre_check_rule(&self) -> &Vec<Box<dyn Rule>>;
    // Check against set of rules before running the event
    fn check_rules(&self) -> Result<bool, BGitError>;
    // Hooks
    fn pre_execute_hook(&self) -> Result<bool, BGitError>;
    fn post_execute_hook(&self) -> Result<bool, BGitError>;
    // Plain execute the event, without any checks and hook
    fn raw_execute(&self) -> Result<bool, BGitError>;

    fn execute(&self) -> Result<bool, BGitError> {
        for rule in self.get_pre_check_rule().iter() {
            let rule_passed = rule.execute()?;
            if !rule_passed {
                return Err(BGitError::new(
                    "Pre-check Rule failed",
                    rule.get_description(),
                    "AtomicEvent",
                    NO_STEP,
                    self.get_name(),
                    rule.get_name(),
                ));
            }
        }
        let pre_event_hook_status = self.pre_execute_hook()?;
        if !pre_event_hook_status {
            return Err(BGitError::new(
                "Pre-event hook failed",
                "Pre-event hook failed!",
                "AtomicEvent",
                NO_STEP,
                self.get_name(),
                NO_RULE,
            ));
        }

        let raw_executor_status = self.raw_execute()?;
        if !raw_executor_status {
            return Err(BGitError::new(
                "Raw executor failed",
                "Raw executor failed!",
                "RawExecutor",
                NO_STEP,
                self.get_name(),
                NO_RULE,
            
            ));
        }

        let post_event_hook_status = self.post_execute_hook()?;
        if !post_event_hook_status {
            return Err(BGitError::new(
                "Post-event hook failed",
                "Post-event hook failed!",
                "AtomicEvent",
                NO_STEP,
                self.get_name(),
                NO_RULE,
            ));
        }
        Ok(true)
    }
}
