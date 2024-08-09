use crate::bgit_error::BGitError;

mod a01_git_install;
mod a02_git_name_email_setup;
mod a03_github_username;
mod a04_gitlab_username;
mod a05_github_credentials_http;
mod a06_gitlab_credentials_http;
mod a07_github_credentials_ssh;
mod a08_gitlab_credentials_ssh;
mod a09_commit_gpg_sign;
mod a11_git_remote_http_ssh;
mod a12_no_secrets_staged;
mod a13_git_lfs;
mod a14_big_repo_size;
mod a15_file_not_gitignored;

#[derive(Debug)]
pub(crate) enum RuleLevel {
    Allow,
    Warning,
    Error,
}

pub(crate) enum RuleOutput {
    Allowed,
    Warning(String),
    Fine,
}

pub(crate) trait Rule {
    fn new(name: &str, description: &str, level: RuleLevel) -> Self
    where
        Self: Sized;
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_level(&self) -> RuleLevel;
    fn execute(&self) -> Result<bool, BGitError>;
    fn check(&self) -> Result<RuleOutput, BGitError>;
    fn apply(&self) -> Result<bool, BGitError>;
    fn verify(&self) -> Result<bool, BGitError>;
}
