
use super::AtomicEvent;
use crate::bgit_error::{BGitError, BGitErrorWorkflowType, NO_EVENT, NO_RULE, NO_STEP};
use crate::rules::Rule;
use git2::{Repository, FetchOptions, Cred, RemoteCallbacks};
use std::env;

pub struct GitPull {
    name: String,
    pre_check_rules: Vec<Box<dyn Rule + Send + Sync>>,
}

impl GitPull {
    pub fn new() -> Self {
        GitPull {
            name: String::from("git_pull"),
            pre_check_rules: Vec::new(),
        }
    }
}

impl AtomicEvent for GitPull {
    fn new() -> Self {
        GitPull::new()
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_action_description(&self) -> &str {
        "Pull latest changes from remote"
    }

    fn add_pre_check_rule(&mut self, rule: Box<dyn Rule + Send + Sync>) {
        self.pre_check_rules.push(rule);
    }

    fn get_pre_check_rule(&self) -> &Vec<Box<dyn Rule + Send + Sync>> {
        &self.pre_check_rules
    }

    fn raw_execute(&self) -> Result<bool, Box<BGitError>> {
        let repo = Repository::discover(".").map_err(|e| {
            Box::new(BGitError::new(
                "Pull Error",
                &format!("Failed to open repository: {}", e),
                BGitErrorWorkflowType::AtomicEvent,
                NO_STEP,
                NO_EVENT,
                NO_RULE,
            ))
        })?;

        let mut remote = repo.find_remote("origin").map_err(|e| {
            Box::new(BGitError::new(
                "Pull Error",
                &format!("Failed to find remote 'origin': {}", e),
                BGitErrorWorkflowType::AtomicEvent,
                NO_STEP,
                NO_EVENT,
                NO_RULE,
            ))
        })?;
let mut cb = RemoteCallbacks::new();
cb.credentials(|_url, username_from_url, _allowed_types| {
    let config = git2::Config::open_default().unwrap();
    Cred::credential_helper(&config, username_from_url.unwrap_or(""), None)
});


        let mut fo = FetchOptions::new();
        fo.remote_callbacks(cb);

        remote.fetch(&["refs/heads/main:refs/remotes/origin/main"], Some(&mut fo), None).map_err(|e| {
            Box::new(BGitError::new(
                "Pull Error",
                &format!("Failed to fetch from remote: {}", e),
                BGitErrorWorkflowType::AtomicEvent,
                NO_STEP,
                NO_EVENT,
                NO_RULE,
            ))
        })?;

        // Merge logic can be added here if needed

        Ok(true)
    }
}