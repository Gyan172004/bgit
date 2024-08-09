use crate::bgit_error::BGitError;
use crate::step::Step;
use crate::step::Task::{ActionStepTask, PromptStepTask};

struct WorkflowQueue {
    name: String,
    init_step: Step,
}

impl WorkflowQueue {
    fn new(name: &str, init_step: Step) -> Self {
        WorkflowQueue {
            name: name.to_owned(),
            init_step,
        }
    }

    fn execute(&self) -> Result<bool, BGitError> {
        if let Step::Start(task) = &self.init_step {
            let mut next_step: Step = match task {
                ActionStepTask(action_step_task) => action_step_task.execute(),
                PromptStepTask(prompt_step_task) => prompt_step_task.execute(),
            };

            while next_step != Step::Stop {
                if let Step::Start(_) = next_step {
                    return Err(BGitError::new(
                        "start step occured in between of workflow",
                        "lal",
                        "idk",
                        "won't tell",
                    ));
                }

                if let Step::Task(task) = next_step {
                    next_step = match task {
                        ActionStepTask(action_step_task) => action_step_task.execute(),
                        PromptStepTask(prompt_step_task) => prompt_step_task.execute()
                    }
                } else {
                    unreachable!("This code is unreachable")
                }
            }
            if next_step == Step::Stop {
                Ok(true)
            } else {
                Err(BGitError::new("stop not found!", "idk", "lmao", "roflmao"))
            }
        } else {
            Err(BGitError::new(
                "init_step must be a Start Task!",
                "PARENT",
                "Initialization",
                "Blah blah",
            ))
        }
    }
}
