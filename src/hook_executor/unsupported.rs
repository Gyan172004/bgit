use std::path::Path;
use crate::bgit_error::BGitError;

pub(crate) fn execute_hook_util(
    _pre_event_hook_path: &Path,
    _event_name: &str,
) -> Result<bool, Box<BGitError>> {
    unimplemented!("Hooks are not supported on this platform");
}