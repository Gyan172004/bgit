use crate::common_store::worflow_store::TASK_IS_GIT_REPO;
use crate::step::{ActionStep, Step, Task};
use crate::workflow_queue::WorkflowQueue;

pub(crate) fn default_cmd_workflow() {
    let workflow_queue = WorkflowQueue::new(
        "Default Workflow",
        Step::Start(Task::ActionStepTask(Box::new(
            TASK_IS_GIT_REPO.copy_struct(),
        ))),
    );
    let result = workflow_queue.execute().unwrap();
    println!("Ran {}", result);
}
