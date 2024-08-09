use crate::step::{ActionStep, Step, Task};
use crate::workflows::default::action::ta01_is_git_repo::IsGitRepo;
use crate::workflow_queue::WorkflowQueue;

pub(crate) fn default_cmd_workflow() {
    let workflow_queue = WorkflowQueue::new(
        "Default Workflow",
        Step::Start(Task::ActionStepTask(Box::new(IsGitRepo::new("is git repo")))),
    );
    let result = workflow_queue.execute().unwrap();
    println!("Ran {}", result)
}
