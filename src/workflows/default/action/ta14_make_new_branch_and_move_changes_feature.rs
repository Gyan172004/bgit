use crate::step::{ActionStep, Step, Task::PromptStepTask};
use git2::{Repository, BranchType};
use std::env;
use std::io::{self, Write};
use crate::workflows::default::prompt::pa04_ask_to_commit::AskToCommit;

pub(crate) struct MakeNewBranchAndMoveChanges {
    name: String,
}

impl MakeNewBranchAndMoveChanges {
    pub(crate) fn new(name: &str) -> Self {
        MakeNewBranchAndMoveChanges {
            name: name.to_owned(),
        }
    }
}

impl ActionStep for MakeNewBranchAndMoveChanges {
    fn new(name: &str) -> Self {
        MakeNewBranchAndMoveChanges::new(name)
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

        // Prompt user for new branch name
        print!("Enter the name for the new branch: ");
        io::stdout().flush().unwrap();
        let mut branch_name = String::new();
        io::stdin().read_line(&mut branch_name).expect("Failed to read input");
        let branch_name = branch_name.trim();

        let head = repo.head().expect("Failed to get HEAD reference");
        let oid = head.target().expect("Failed to get HEAD OID");
        let parent_commit = repo.find_commit(oid).expect("Failed to find parent commit");

        // Create the new branch and switch to it
        match repo.branch(branch_name, &parent_commit, BranchType::Local) {
            Ok(_) => {
                repo.checkout_head(None).expect("Failed to checkout new branch");
                
                // Stash changes
                repo.stash_save("Stashing changes before creating a new branch", None, None).expect("Failed to stash changes");

                // Apply the stash on the new branch
                repo.stash_apply(0, None).expect("Failed to apply stash");
                repo.stash_drop(0).expect("Failed to drop stash");
                
                println!("Changes have been moved to the new branch.");
                
                // Return prompt to commit the changes
                Step::Task(PromptStepTask(Box::new(AskToCommit::new("ask to commit"))))
            }
            Err(e) => {
                eprintln!("Failed to create new branch: {}", e);
                Step::Stop
            }
        }
    }
}
