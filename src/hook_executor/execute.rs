use crate::bgit_error::{BGitError, NO_RULE, NO_STEP};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{
    io::{BufRead, BufReader},
    thread,
};

pub(crate) fn execute_hook_util(
    pre_event_hook_path: &Path,
    event_name: &str,
) -> Result<bool, Box<BGitError>> {
    if pre_event_hook_path.exists() {
        let pre_event_hook_path_str = pre_event_hook_path.to_str().unwrap();

        #[cfg(windows)]
        unimplemented!("Windows is not supported yet"); // TODO: Implement for Windows

        let metdata = fs::metadata(pre_event_hook_path).expect("Failed to get hook file metadata!");
        let mut permissions = metdata.permissions();

        // Check if the file is already executable
        if permissions.mode() & 0o111 == 0 {
            // File is not executable, so make it executable
            permissions.set_mode(permissions.mode() | 0o755); // 0o755 gives rwxr-xr-x permissions
            fs::set_permissions(pre_event_hook_path, permissions)
                .expect("Failed to make event hook exectable!");
        }

        // Spawn the command
        let mut child = Command::new(pre_event_hook_path_str)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to run event-hook");

        // Take ownership of stdout and stderr
        let stdout = child.stdout.take().expect("Failed to capture stdout");
        let stderr = child.stderr.take().expect("Failed to capture stderr");

        // Stream stdout
        let stdout_handle = thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line_unwrap) = line {
                    println!("{}", line_unwrap);
                }
            }
        });

        // Stream stderr
        let stderr_handle = thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line_unwrap) = line {
                    eprintln!("{}", line_unwrap);
                }
            }
        });

        // Wait for the command to finish
        let status = child.wait().expect("Failed to wait on child");

        // Wait for the streaming threads to finish
        stdout_handle.join().expect("Failed to join stdout thread");
        stderr_handle.join().expect("Failed to join stderr thread");

        if status.success() {
            Ok(true)
        } else {
            Err(Box::new(BGitError::new(
                "event-hook failed",
                &format!(
                    "Event-hook exited with non-zero status {}\nFile:{}",
                    status.code().unwrap_or(-1),
                    pre_event_hook_path_str
                ),
                "AtomicEvent",
                NO_STEP,
                event_name,
                NO_RULE,
            )))
        }
    } else {
        // No pre-event-hook found, so return true as well!
        Ok(true)
    }
}
