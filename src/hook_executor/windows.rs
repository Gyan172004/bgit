//! Hook execution utilities for Windows systems.
//!
//! This module provides functionality to execute Git hooks on Windows environments,
//! supporting multiple execution methods and file types.
//!
//! # Features
//! - Supports multiple script extensions (.bat, .cmd, .ps1, .exe)
//! - Handles bash scripts through Git Bash or WSL
//! - Provides fallback mechanisms for different execution environments
//!
//! # Extension Precedence
//! The module checks for hook files in the following order:
//! 1. Shell scripts with no extension (direct executable)
//! 2. .bat (Batch files)
//! 3. .cmd (Command files)
//! 4. .ps1 (PowerShell scripts)
//! 5. .exe (Executable files)
//!
//! # Execution Strategy
//! The module follows this hierarchy for executing hooks:
//! 1. Direct execution based on file extension:
//!    - .ps1 -> PowerShell with bypass policy
//!    - .bat/.cmd -> cmd.exe
//!    - .exe -> direct execution
//! 2. Bash execution (if shebang detected):
//!    - Git Bash (from Program Files): 'C:\Program Files\Git\bin\bash.exe' or 'C:\Program Files (x86)\Git\bin\bash.exe'
//!    - MSYS2 bash: 'C:\msys64\usr\bin\bash.exe'
//!    - bash.exe (if in PATH)
//!    - WSL bash
//! 3. Fallback to cmd.exe as last resort
//!
//! # Functions
//!
//! ## `find_hook_with_extension`
//! Searches for a hook file with supported extensions in precedence order.
//!
//! ## `create_command_for_hook`
//! Creates an appropriate Command instance based on the hook's file extension.
//!
//! ## `try_bash_execution`
//! Attempts to execute a hook using bash, searching in common installation locations.
//!
//! ## `execute_hook_util`
//! Main function for executing hooks, handling the execution process and results.
//!
//! # Errors
//! - Returns `BGitError` for hook execution failures
//! - Provides detailed error context including:
//!   - Hook execution failures
//!   - Process spawn errors
//!   - Exit code failures
//!   - Path validation errors
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

use super::error::create_hook_error;
use super::process::handle_process_output;
use crate::bgit_error::BGitError;

fn create_command_for_hook(hook_path: &Path) -> Option<Command> {
    let extension = hook_path.extension()?.to_str()?;

    match extension {
        "ps1" => {
            let mut cmd = Command::new("powershell");
            cmd.args(["-ExecutionPolicy", "Bypass", "-File", hook_path.to_str()?]);
            Some(cmd)
        }
        "bat" | "cmd" => {
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", hook_path.to_str()?]);
            Some(cmd)
        }
        "exe" => Some(Command::new(hook_path)),
        _ => Some(Command::new(hook_path)),
    }
}

fn try_bash_execution(hook_path: &Path) -> Option<Command> {
    // Try to read the file to check for shebang
    let file = fs::File::open(hook_path).ok()?;
    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    let _ = reader.read_line(&mut first_line);

    if !first_line.contains("bash") {
        return None;
    }

    // Try to find bash in common locations
    let bash_paths = [
        "C:\\Program Files\\Git\\bin\\bash.exe",
        "C:\\Program Files (x86)\\Git\\bin\\bash.exe",
        "C:\\msys64\\usr\\bin\\bash.exe",
        "bash.exe", // If it's in PATH
    ];

    for &bash_path in &bash_paths {
        if Path::new(bash_path).exists() || bash_path == "bash.exe" {
            let mut cmd = Command::new(bash_path);
            cmd.arg(hook_path.to_str()?);
            return Some(cmd);
        }
    }

    // If bash not found, try WSL
    if Command::new("wsl").arg("--version").output().is_ok() {
        let mut cmd = Command::new("wsl");
        cmd.arg(hook_path.to_str()?);
        return Some(cmd);
    }

    None
}

pub fn execute_hook_util(event_hook_path: &Path, event_name: &str) -> Result<bool, Box<BGitError>> {
    // Check if hook exists with any of the supported extensions
    if !event_hook_path.exists() {
        return Ok(true);
    }

    // Try to create an appropriate command for the hook
    let mut command = match create_command_for_hook(&event_hook_path) {
        Some(cmd) => cmd,
        None => {
            // Try bash execution as fallback
            match try_bash_execution(event_hook_path) {
                Some(cmd) => cmd,
                None => {
                    // Last resort: try cmd.exe
                    let mut cmd = Command::new("cmd");
                    cmd.args([
                        "/C",
                        event_hook_path.to_str().ok_or_else(|| {
                            create_hook_error(
                                "Invalid path",
                                "Path contains invalid characters",
                                event_name,
                            )
                        })?,
                    ]);
                    cmd
                }
            }
        }
    };

    // Configure command to capture output
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    // Execute the command
    let mut child = command
        .spawn()
        .map_err(|e| create_hook_error("Failed to execute hook", &e.to_string(), event_name))?;

    // Handle stdout and stderr
    handle_process_output(&mut child)?;

    // Wait for the command to complete
    let status = child.wait().map_err(|e| {
        create_hook_error(
            "Failed to wait for hook execution",
            &e.to_string(),
            event_name,
        )
    })?;

    if status.success() {
        Ok(true)
    } else {
        Err(create_hook_error(
            "event-hook failed",
            &format!(
                "Hook for {} failed with exit code: {:?}",
                event_name,
                status.code()
            ),
            event_name,
        ))
    }
}
