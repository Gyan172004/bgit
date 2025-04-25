use crate::step::{ActionStep, Step, Task};
use crate::workflow_queue::WorkflowQueue;
use crate::workflows::default::action::ta01_is_git_repo::IsGitRepo;

pub(crate) fn default_cmd_workflow() {
    let workflow_queue = WorkflowQueue::new(Step::Start(Task::ActionStepTask(Box::new(
        IsGitRepo::new(),
    ))));
    match workflow_queue.execute() {
        Ok(_) => {}
        Err(err) => err.print_error(),
    };
}
