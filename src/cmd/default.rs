use crate::common_store::workflow_store::TASK_IS_GIT_REPO;
use crate::step::{ActionStep, Step, Task};
use crate::workflow_queue::WorkflowQueue;

pub(crate) fn default_cmd_workflow() {
    let workflow_queue = WorkflowQueue::new(
        "Default Workflow",
        Step::Start(Task::ActionStepTask(Box::new(
            TASK_IS_GIT_REPO.copy_struct(),
        ))),
    );
    match workflow_queue.execute() {
        Ok(_) => {}
        Err(err) => eprintln!("Error : {:#?}", err),
    };
}
