use crate::{
    bgit_error::BGitError,
    common_store::workflow_store::TASK_COMMIT_CHANGES,
    step::{ActionStep, Step, Task::ActionStepTask},
};
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use git2::Repository;
use std::env;
use std::fs;
use std::path::PathBuf;

// Define some constants for error codes, event codes, and rule codes
const ERROR_CODE_REPO_OPEN: u32 = 1001;
const ERROR_CODE_ADD_FILES: u32 = 1004;

const EVENT_CODE_ADD_MANUAL: u32 = 2004;

const RULE_CODE_ADD_FILES: u32 = 3004;

pub(crate) struct AddManualFiles {
    name: String,
}

impl ActionStep for AddManualFiles {
    fn new(name: &str) -> Self
    where
        Self: Sized,
    {
        AddManualFiles {
            name: name.to_owned(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Result<Step, Box<BGitError>> {
        let cwd = env::current_dir().expect("Failed to get current directory");
        let repo = match Repository::discover(&cwd) {
            Ok(repo) => repo,
            Err(e) => {
                return Err(Box::new(BGitError::new(
                    "Failed to open Git repository",
                    &format!("Git2 Error: {}", e),
                    "AddManualFiles",
                    ERROR_CODE_REPO_OPEN,
                    EVENT_CODE_ADD_MANUAL,
                    RULE_CODE_ADD_FILES,
                )));
            }
        };

        let files = get_unstaged_files(&repo, &cwd)?;

        if files.is_empty() {
            println!("No unstaged files found.");
            return Ok(Step::Task(ActionStepTask(Box::new(
                TASK_COMMIT_CHANGES.copy_struct(),
            ))));
        }

        let selected_files = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select files to add")
            .items(&files)
            .interact()
            .expect("Failed to get user selection");

        let mut index = match repo.index() {
            Ok(index) => index,
            Err(e) => {
                return Err(Box::new(BGitError::new(
                    "Failed to get repository index",
                    &format!("Git2 Error: {}", e),
                    "AddManualFiles",
                    ERROR_CODE_ADD_FILES,
                    EVENT_CODE_ADD_MANUAL,
                    RULE_CODE_ADD_FILES,
                )));
            }
        };

        for &idx in selected_files.iter() {
            let file = &files[idx];
            if let Err(e) = index.add_path(&PathBuf::from(file)) {
                return Err(Box::new(BGitError::new(
                    &format!("Failed to add file: {}", file),
                    &format!("Git2 Error: {}", e),
                    "AddManualFiles",
                    ERROR_CODE_ADD_FILES,
                    EVENT_CODE_ADD_MANUAL,
                    RULE_CODE_ADD_FILES,
                )));
            }
        }

        if let Err(e) = index.write() {
            return Err(Box::new(BGitError::new(
                "Failed to write index",
                &format!("Git2 Error: {}", e),
                "AddManualFiles",
                ERROR_CODE_ADD_FILES,
                EVENT_CODE_ADD_MANUAL,
                RULE_CODE_ADD_FILES,
            )));
        }

        println!("Selected files have been added to the staging area.");
        Ok(Step::Task(ActionStepTask(Box::new(
            TASK_COMMIT_CHANGES.copy_struct(),
        ))))
    }
}

fn get_unstaged_files(repo: &Repository, cwd: &PathBuf) -> Result<Vec<String>, Box<BGitError>> {
    let mut unstaged_files = Vec::new();
    let statuses = repo
        .statuses(None)
        .map_err(|e| Box::new(BGitError::new(
            "Failed to get repository status",
            &format!("Git2 Error: {}", e),
            "AddManualFiles",
            ERROR_CODE_REPO_OPEN,
            EVENT_CODE_ADD_MANUAL,
            RULE_CODE_ADD_FILES,
        )))?;

    for entry in statuses.iter() {
        let status = entry.status();
        let path = entry.path().unwrap_or("").to_string();

        if status.is_wt_new() || status.is_wt_modified() {
            unstaged_files.push(path);
        }
    }

    Ok(unstaged_files)
}
