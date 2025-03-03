use crate::bgit_error::BGitError;
use std::path::Path;

pub(crate) fn execute_hook_util(
    _event_hook_path: &Path,
    _event_name: &str,
) -> Result<bool, Box<BGitError>> {
    unimplemented!("Hooks are not supported on this platform");
}
