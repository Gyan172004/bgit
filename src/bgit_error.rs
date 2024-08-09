
pub struct BGitError {
    name: String,
    workflow_name: String,
    step_name: String,
    message: String
}

impl BGitError {
    pub fn new(name: &str, workflow_name: &str, step_name: &str, message: &str) -> Self {
        Self {
            name: name.to_owned(),
            workflow_name: workflow_name.to_owned(),
            step_name: step_name.to_owned(),
            message: message.to_owned()
        }
    }
}
