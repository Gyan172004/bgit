use crate::events::BgitEvent;

mod a11_git_remote_http_ssh;
mod a03_github_username;
mod a09_commit_gpg_sign;
mod a01_git_install;
mod a02_git_name_email_setup;
mod a05_github_credentials_http;
mod a07_github_credentials_ssh;
mod a06_gitlab_credentials_http;
mod a08_gitlab_credentials_ssh;
mod a04_gitlab_username;
mod a12_no_secrets_staged;
mod a13_git_lfs;
mod a14_big_repo_size;
mod a15_file_not_gitignored;

pub enum BgitRuleLevel {
    Allow,
    Warning,
    Error
}

pub enum BgitRuleOutput {
    Allowed,
    Warning(String),
    Fine
}

pub trait BgitRule {
    fn new(name: String, id: u32, level: BgitRuleLevel, events: Vec<Box<dyn BgitEvent>>) -> Self where Self: Sized;
    fn get_name(&self) -> String;
    fn get_id(&self) -> u32;
    fn get_level(&self) -> BgitRuleLevel;
    fn get_event(&self) -> Vec<Box<dyn BgitEvent>>;
    fn check(&self) -> Result<BgitRuleOutput, String>;
    fn apply(&self) -> Result<bool, &str>;
    fn verify(&self) -> Result<bool, &str>;
}

