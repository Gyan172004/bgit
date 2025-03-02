use crate::bgit_error::{BGitError, NO_RULE, NO_STEP};

pub fn create_hook_error(message: &str, details: &str, event_name: &str) -> Box<BGitError> {
    Box::new(BGitError::new(
        message,
        details,
        "AtomicEvent",
        NO_STEP,
        event_name,
        NO_RULE,
    ))
}