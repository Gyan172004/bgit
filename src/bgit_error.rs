#[derive(Debug)]
pub(crate) struct BGitError {
    name: String,
    message: String,
    workflow_name: String,
    step_name: String,
    event_name: String,
    rule_name: String,
}

pub(crate) const EMPTY_STRING: &str = "";
pub(crate) const NO_STEP: &str = EMPTY_STRING;
pub(crate) const NO_EVENT: &str = EMPTY_STRING;
pub(crate) const NO_RULE: &str = EMPTY_STRING;

impl BGitError {
    pub(crate) fn new(
        name: &str,
        message: &str,
        workflow_name: &str,
        step_name: &str,
        event_name: &str,
        rule_name: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            message: message.to_owned(),
            workflow_name: workflow_name.to_owned(),
            step_name: step_name.to_owned(),
            event_name: event_name.to_owned(),
            rule_name: rule_name.to_owned(),
        }
    }

    pub(crate) fn print_error(&self) {
        eprintln!("The operation errored out for some reasons!");
        eprint!("[");
        eprint!("{}", self.workflow_name);
        eprint!("::");
        if self.step_name != NO_STEP {
            eprint!("{}", self.step_name);
        }
        eprint!("::");
        if self.event_name != NO_EVENT {
            eprint!("{}", self.event_name);
        }
        eprint!("::");
        if self.rule_name != NO_RULE {
            eprint!("{}", self.rule_name);
        }
        eprint!("] ");

        eprintln!("{}", self.name);
        eprintln!("Message: {}", self.message);
    }
}
