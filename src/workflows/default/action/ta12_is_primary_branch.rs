use crate::step::{ActionStep, Step, PromptStep, Task::PromptStepTask};
use crate::workflows::default::prompt::pa;
use crate::workflows::default::prompt::pa07_ask_same_feature::AskSameFeature;
use git2::{Repository, Reference};
use std::env;

pub(crate) struct IsPrimaryBranch {
    name: String,
}

impl IsPrimaryBranch {
    pub(crate) fn new(name: &str) -> Self {
        IsPrimaryBranch {
            name: name.to_owned(),
        }
    }
}

impl ActionStep for IsPrimaryBranch {
    fn new(name: &str) -> Self {
        IsPrimaryBranch::new(name)
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Step {
        let cwd = env::current_dir().expect("Failed to get current directory");
        let repo = match Repository::open(&cwd) {
            Ok(repo) => repo,
            Err(e) => {
                eprintln!("Failed to open Git repository: {}", e);
                return Step::Stop;
            }
        };

        let head_ref = match repo.head() {
            Ok(reference) => reference,
            Err(e) => {
                eprintln!("Failed to get HEAD reference: {}", e);
                return Step::Stop;
            }
        };

        let current_branch = match head_ref.shorthand() {
            Some(name) => name,
            None => {
                eprintln!("Failed to get current branch name.");
                return Step::Stop;
            }
        };

        let is_primary_branch = matches!(current_branch, "main" | "master" | "dev");

        if is_primary_branch {
            println!("You are on a primary branch: {}", current_branch);
            Step::Task(PromptStepTask(Box::new(AskIsSoleContributor::new("ask if sole contributor"))))
        } else {
            println!("You are not on a primary branch: {}", current_branch);
            Step::Task(PromptStepTask(Box::new(AskSameFeature::new("ask if same feature"))))
        }
    }
}
