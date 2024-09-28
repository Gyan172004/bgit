use crate::step::{ActionStep, Step};
use git2::{Repository, BranchType, ResetType};
use std::env;
use std::io::{self, Write};

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

        let mut index = repo.index().expect("Failed to get repository index");
        let tree_id = index.write_tree().expect("Failed to write tree");
        let tree = repo.find_tree(tree_id).expect("Failed to find tree");

        let head = repo.head().expect("Failed to get HEAD reference");
        let oid = head.target().expect("Failed to get HEAD OID");
        let parent_commit = repo.find_commit(oid).expect("Failed to find parent commit");
        let parents = vec![parent_commit];

        // Create the new branch and switch to it
        match repo.branch(branch_name, &repo.head().unwrap().peel_to_commit().unwrap(), BranchType::Local) {
            Ok(_) => {
                repo.checkout_head(Some(git2::Build::new().force()), None).expect("Failed to checkout new branch");
                println!("Switched to new branch: {}", branch_name);
                
                // Stash changes
                repo.stash_save("Stashing changes before creating a new branch", None, None).expect("Failed to stash changes");
                
                // Pop the stash on the new branch
                repo.stash_apply(0, None).expect("Failed to apply stash");
                repo.stash_drop(0).expect("Failed to drop stash");
                
                println!("Changes have been moved to new branch.");
                Step::Stop // or proceed to the next step
            }
            Err(e) => {
                eprintln!("Failed to create new branch: {}", e);
                Step::Stop
            }
        }
    }
}
