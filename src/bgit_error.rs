pub(crate) struct BGitError {
    name: String,
    workflow_name: String,
    step_name: String,
    message: String,
}

impl BGitError {
    pub(crate) fn new(name: &str, workflow_name: &str, step_name: &str, message: &str) -> Self {
        Self {
            name: name.to_owned(),
            workflow_name: workflow_name.to_owned(),
            step_name: step_name.to_owned(),
            message: message.to_owned(),
        }
    }

    pub(crate) fn print_error(&self) {
        eprintln!("The command errored out for some reasons!\nName: {}\nWorflow Name: {}\nStep Name: {}\nMessage: {}", self.name, self.workflow_name, self.step_name, self.message);
    }
}
