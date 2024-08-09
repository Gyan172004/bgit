#[derive(PartialEq)]
pub(crate) enum Step {
    Start(Task),
    Stop,
    Task(Task),
}

pub(crate) enum Task {
    ActionStepTask(Box<dyn ActionStep>),
    PromptStepTask(Box<dyn PromptStep>),
}

impl std::cmp::PartialEq for Task {
    fn eq(&self, _rhs: &Task) -> bool {
        // Disabled compare matching for Task enum
        todo!("Implement Task PartialEq Matching for dyn types")
    }
}

pub(crate) trait ActionStep {
    fn new(name: &str) -> Self
    where
        Self: Sized;
    fn get_name(&self) -> &str;
    fn execute(&self) -> Step;
}

pub(crate) trait PromptStep {
    fn new(name: &str) -> Self
    where
        Self: Sized;
    fn get_name(&self) -> &str;
    fn execute(&self) -> Step;
}
