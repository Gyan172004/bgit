use crate::step::{ActionStep, Step, Task::ActionStepTask};
use git2::{Repository, Signature};
use std::env;
use crate::workflows::default::prompt::pa05_ask_to_pull_or_push::AskToPullOrPush;
use std::io::{self, Write};

pub(crate) struct Commit {
    name: String,
}

impl Commit {
    pub(crate) fn new(name: &str) -> Self {
        Commit {
            name: name.to_owned(),
        }
    }
}

impl ActionStep for Commit {
    fn new(name: &str) -> Self {
        Commit::new(name)
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

        // Prompt user for commit message
        print!("Enter a commit message: ");
        io::stdout().flush().unwrap();
        let mut commit_msg = String::new();
        io::stdin().read_line(&mut commit_msg).expect("Failed to read input");
        let commit_msg = commit_msg.trim();

        let signature = Signature::now("User", "user@example.com").expect("Failed to create signature");
        let mut index = repo.index().expect("Failed to get repository index");

        // Create the commit
        let tree_id = index.write_tree().expect("Failed to write tree");
        let tree = repo.find_tree(tree_id).expect("Failed to find tree");
        
        // Get the current HEAD and find the parent commit
        let parent_commit = repo.head().ok()
            .and_then(|head| head.target())
            .and_then(|oid| repo.find_commit(oid).ok());

        // Collect parent commits into a vector
        let parents = if let Some(commit) = parent_commit {
            vec![commit]
        } else {
            vec![]
        };

        // Convert to a slice of references
        let parent_refs: Vec<_> = parents.iter().collect();

        match repo.commit(
            Some("HEAD"), 
            &signature, 
            &signature, 
            commit_msg, 
            &tree, 
            parent_refs.as_slice()  // Pass the slice of references
        ) {
            Ok(_) => {
                println!("Changes have been committed.");
                Step::Stop
                // Step::Task(ActionStepTask(Box::new(AskToPullOrPush::new("ask to pull/push"))))
            }
            Err(e) => {
                eprintln!("Failed to commit changes: {}", e);
                Step::Stop
            }
        }
    }
}
