use git2::Repository;
use std::env;

use crate::{
    bgit_error::{BGitError, NO_RULE, NO_STEP},
    hook_executor::execute::execute_hook_util,
    rules::Rule,
};
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
    // Plain execute the event, without any checks and hook
    fn raw_execute(&self) -> Result<bool, Box<BGitError>>;

    // Hooks
    fn pre_execute_hook(&self) -> Result<bool, Box<BGitError>> {
        let pre_event_hook_file_name: String = format!("pre-{}", self.get_name());
        self.execute_hook(&pre_event_hook_file_name)
    }
    fn post_execute_hook(&self) -> Result<bool, Box<BGitError>> {
        let post_event_hook_file_name: String = format!("post-{}", self.get_name());
        self.execute_hook(&post_event_hook_file_name)
    }
    /// Run hooks inside `{RepositoryBase}/.bgit/hooks/[pre|post]-{hook_name}`
    /// TODO: Implement for Windows and other OS using custom toml for custom runtime like
    /// shell and languages
    fn execute_hook(&self, pre_event_hook_file_name: &str) -> Result<bool, Box<BGitError>> {
        let cwd = env::current_dir().expect("Failed to get current directory");
        let git_repo = Repository::discover(cwd);
        if git_repo.is_ok() {
            let git_repo = git_repo.unwrap();
            let git_repo_path = git_repo
                .path()
                .parent()
                .expect("Failed to crawl to parent directory of .git folder");
            let bgit_hooks_path = git_repo_path.join(".bgit").join("hooks");

            let pre_event_hook_path = bgit_hooks_path.join(pre_event_hook_file_name);
            execute_hook_util(&pre_event_hook_path, self.get_name())
        } else {
            Err(Box::new(BGitError::new(
                "Can't run event-hook",
                "No git repository found",
                "AtomicEvent",
                NO_STEP,
                self.get_name(),
                NO_RULE,
            )))
        }
    }

    // Check against set of rules before running the event
    fn check_rules(&self) -> Result<bool, Box<BGitError>> {
        for rule in self.get_pre_check_rule().iter() {
            let rule_passed = rule.execute()?;
            if !rule_passed {
                return Err(Box::new(BGitError::new(
                    "Pre-check Rule failed",
                    rule.get_description(),
                    "AtomicEvent",
                    NO_STEP,
                    self.get_name(),
                    rule.get_name(),
                )));
            }
        }
        Ok(true)
    }

    fn execute(&self) -> Result<bool, Box<BGitError>> {
        let rule_check_status = self.check_rules()?;
        if !rule_check_status {
            return Ok(false);
        }
        let pre_event_hook_status = self.pre_execute_hook()?;
        if !pre_event_hook_status {
            return Err(Box::new(BGitError::new(
                "Pre-event hook failed",
                "Pre-event hook failed!",
                "AtomicEvent",
                NO_STEP,
                self.get_name(),
                NO_RULE,
            )));
        }

        let raw_executor_status = self.raw_execute()?;
        if !raw_executor_status {
            return Err(Box::new(BGitError::new(
                "Raw executor failed",
                "Raw executor failed!",
                "RawExecutor",
                NO_STEP,
                self.get_name(),
                NO_RULE,
            )));
        }

        let post_event_hook_status = self.post_execute_hook()?;
        if !post_event_hook_status {
            return Err(Box::new(BGitError::new(
                "Post-event hook failed",
                "Post-event hook failed!",
                "AtomicEvent",
                NO_STEP,
                self.get_name(),
                NO_RULE,
            )));
        }
        Ok(true)
    }
}
