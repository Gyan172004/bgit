use crate::rules::BgitRule;
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

/// List of various Git Events to be called with git2-rs library
pub enum GitEvent {
    Init,
    Add,
    Commit,
    Push,
    Pull,
    Rebase,
    Merge,
    Log,
    Status
}
/// Sample schema for an Events
/// ```rs
/// struct <event name> {
///     name: String,
///     action_description: String,
///     id: u32,
///     args: Vec<>
/// }
/// ```
pub(crate) trait BgitEvent {
    fn new(name: String, action_description: String, id: u32, args: Vec<&str>) -> Self where Self: Sized;
    fn get_name(&self) -> String;
    fn get_action_description(&self) -> String;
    fn get_id(&self) -> u32;
    fn get_type(&self) -> Vec<GitEvent>;
    fn get_rules(&self) -> Vec<Box<dyn BgitRule>>;
    fn apply(&self) -> Result<bool, &str>;
}
