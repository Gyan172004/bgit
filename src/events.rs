use crate::{bgit_error::BGitError, rules::Rule};
mod git_add;
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
///     description: String,
///     pre_check_rules: Vec<dyn Rule>
/// }
/// List of various Git Events to be called with git2-rs library
pub(crate) trait AtomicEvent {
    fn new(name: String, action_description: String) -> Self
    where
        Self: Sized;
    fn get_name(&self) -> String;
    fn get_action_description(&self) -> String;
    fn check_rules(&self) -> Result<bool, BGitError>;
    fn add_pre_check_rule(&self, rule: dyn Rule);
    fn get_pre_check_rule(&self) -> Vec<Box<dyn Rule>>;
    fn pre_execute_hook(&self) -> Result<bool, BGitError>;
    fn execute(&self) -> Result<bool, BGitError>;
    fn post_execute_hook(&self) -> Result<bool, BGitError>;
}
