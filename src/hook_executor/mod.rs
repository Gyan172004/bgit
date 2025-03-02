mod error;
mod process;

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;
#[cfg(not(any(windows, unix)))]
mod unsupported;

// Re-export the main function
// Note: We're only exporting execute_hook_util, not create_hook_error
#[cfg(unix)]
pub use self::unix::execute_hook_util;
#[cfg(windows)]
pub use self::windows::execute_hook_util;
#[cfg(not(any(windows, unix)))]
pub use self::unsupported::execute_hook_util;