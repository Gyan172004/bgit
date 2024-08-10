use crate::bgit_error::{BGitError, NO_EVENT, NO_RULE, NO_STEP};
use crate::step::Step;
use crate::step::Task::{ActionStepTask, PromptStepTask};

pub(crate) struct WorkflowQueue {
    name: String,
    init_step: Step,
}

impl WorkflowQueue {
    pub(crate) fn new(name: &str, init_step: Step) -> Self {
        WorkflowQueue {
            name: name.to_owned(),
            init_step,
        }
    }

    pub(crate) fn execute(&self) -> Result<bool, BGitError> {
        if let Step::Start(task) = &self.init_step {
            let mut next_step: Step = match task {
                ActionStepTask(action_step_task) => action_step_task.execute(),
                PromptStepTask(prompt_step_task) => prompt_step_task.execute(),
            };

            while next_step != Step::Stop {
                if let Step::Start(_) = next_step {
                    return Err(BGitError::new(
                        "next_step must not be a Start Task!", 
                        "next_step must not be a Start Task! This is a bug in the code",
                        "WorkflowQueue",
                        NO_STEP,
                        NO_EVENT,
                        NO_RULE
                    ));
                }

                if let Step::Task(task) = next_step {
                    next_step = match task {
                        ActionStepTask(action_step_task) => action_step_task.execute(),
                        PromptStepTask(prompt_step_task) => prompt_step_task.execute(),
                    }
                } else {
                    unreachable!("This code is unreachable")
                }
            }
            if next_step == Step::Stop {
                Ok(true)
            } else {
                Err(BGitError::new(
                    "final_step must be a Stop Task!",
                    "final_step must be a Stop Task! This is a bug in the code",
                    "WorkflowQueue",
                    NO_STEP,
                    NO_EVENT,
                    NO_RULE
                ))
            }
        } else {
            Err(BGitError::new(
                "init_step must be a Start Task!",
                "init_step must be a Start Task! This is a bug in the code",
                "WorkflowQueue",
                NO_STEP,
                NO_EVENT,
                NO_RULE
            ))
        }
    }
}
