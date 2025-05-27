use super::AtomicEvent;
use crate::bgit_error::{BGitError, BGitErrorWorkflowType, NO_EVENT, NO_RULE, NO_STEP};
use crate::rules::Rule;
use git2::{Repository, Branch, BranchType};

pub struct GitBranch {
    name: String,
    pre_check_rules: Vec<Box<dyn Rule + Send + Sync>>,
    branch_name: String,
    delete_flag: bool,
    list_flag: bool,
}

impl GitBranch {
    pub fn set_branch_name(&mut self, name: &str) -> &mut Self {
        self.branch_name = name.to_string();
        self
    }

    pub fn set_delete_flag(&mut self, flag: bool) -> &mut Self {
        self.delete_flag = flag;
        self
    }

    pub fn set_list_flag(&mut self, flag: bool) -> &mut Self {
        self.list_flag = flag;
        self
    }
}

impl AtomicEvent for GitBranch {
    fn new() -> Self {
        GitBranch {
            name: String::from("git_branch"),
            pre_check_rules: Vec::new(),
            branch_name: String::new(),
            delete_flag: false,
            list_flag: false,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_action_description(&self) -> &str {
        "Manage Git branches"
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
                "Branch Error",
                &format!("Failed to open repository: {}", e),
                BGitErrorWorkflowType::AtomicEvent,
                NO_STEP,
                NO_EVENT,
                NO_RULE,
            ))
        })?;

        if self.list_flag {
            // List all branches
            let branches = repo.branches(None).map_err(|e| {
                Box::new(BGitError::new(
                    "Branch Error",
                    &format!("Failed to list branches: {}", e),
                    BGitErrorWorkflowType::AtomicEvent,
                    NO_STEP,
                    NO_EVENT,
                    NO_RULE,
                ))
            })?;

            println!("Branches:");
            for branch_result in branches {
                let (branch, branch_type) = branch_result.map_err(|e| {
                    Box::new(BGitError::new(
                        "Branch Error",
                        &format!("Failed to get branch info: {}", e),
                        BGitErrorWorkflowType::AtomicEvent,
                        NO_STEP,
                        NO_EVENT,
                        NO_RULE,
                    ))
                })?;

                let name = branch.name().map_err(|e| {
                    Box::new(BGitError::new(
                        "Branch Error",
                        &format!("Failed to get branch name: {}", e),
                        BGitErrorWorkflowType::AtomicEvent,
                        NO_STEP,
                        NO_EVENT,
                        NO_RULE,
                    ))
                })?;

                println!("  {} ({:?})", name.unwrap_or("unnamed"), branch_type);
            }
        } else if self.delete_flag {
            // Delete branch
            if self.branch_name.is_empty() {
                return Err(Box::new(BGitError::new(
                    "Branch Error",
                    "Branch name not specified for deletion",
                    BGitErrorWorkflowType::AtomicEvent,
                    NO_STEP,
                    NO_EVENT,
                    NO_RULE,
                )));
            }

            let mut branch = repo.find_branch(&self.branch_name, BranchType::Local).map_err(|e| {
                Box::new(BGitError::new(
                    "Branch Error",
                    &format!("Failed to find branch: {}", e),
                    BGitErrorWorkflowType::AtomicEvent,
                    NO_STEP,
                    NO_EVENT,
                    NO_RULE,
                ))
            })?;

            branch.delete().map_err(|e| {
                Box::new(BGitError::new(
                    "Branch Error",
                    &format!("Failed to delete branch: {}", e),
                    BGitErrorWorkflowType::AtomicEvent,
                    NO_STEP,
                    NO_EVENT,
                    NO_RULE,
                ))
            })?;
        } else {
            // Create new branch
            if self.branch_name.is_empty() {
                return Err(Box::new(BGitError::new(
                    "Branch Error",
                    "Branch name not specified for creation",
                    BGitErrorWorkflowType::AtomicEvent,
                    NO_STEP,
                    NO_EVENT,
                    NO_RULE,
                )));
            }

            let head = repo.head().map_err(|e| {
                Box::new(BGitError::new(
                    "Branch Error",
                    &format!("Failed to get HEAD: {}", e),
                    BGitErrorWorkflowType::AtomicEvent,
                    NO_STEP,
                    NO_EVENT,
                    NO_RULE,
                ))
            })?;

            let commit = head.peel_to_commit().map_err(|e| {
                Box::new(BGitError::new(
                    "Branch Error",
                    &format!("Failed to get commit: {}", e),
                    BGitErrorWorkflowType::AtomicEvent,
                    NO_STEP,
                    NO_EVENT,
                    NO_RULE,
                ))
            })?;

            repo.branch(&self.branch_name, &commit, false).map_err(|e| {
                Box::new(BGitError::new(
                    "Branch Error",
                    &format!("Failed to create branch: {}", e),
                    BGitErrorWorkflowType::AtomicEvent,
                    NO_STEP,
                    NO_EVENT,
                    NO_RULE,
                ))
            })?;
        }

        Ok(true)
    }
}