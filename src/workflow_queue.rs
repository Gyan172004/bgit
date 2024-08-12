use crate::bgit_error::{BGitError, NO_EVENT, NO_RULE, NO_STEP};
use crate::step::Step;
use crate::step::Task::{ActionStepTask, PromptStepTask};
use colored::Colorize;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::time::Duration;
use std::time::Instant;

const HATCHING_CHICK_EMOJI: &str = "🐣";

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

    pub(crate) fn execute(&self) -> Result<bool, Box<BGitError>> {
        if let Step::Start(task) = &self.init_step {
            let started = Instant::now();
            // Initialize spinner for progress indication
            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(500));
            pb.set_style(
                ProgressStyle::with_template(
                    "{spinner:.cyan/blue.bold} [{pos:.yellow}/?] Executing step: {wide_msg:.green}",
                )
                .unwrap(),
            );

            pb.inc(1);
            pb.tick();

            let mut next_step: Step = match task {
                ActionStepTask(action_step_task) => {
                    eprintln!(
                        "{} Running Action Step: {}",
                        HATCHING_CHICK_EMOJI,
                        action_step_task.get_name().cyan().bold()
                    );
                    pb.set_message(format!(
                        "Step '{}' in progress...",
                        action_step_task.get_name().bold()
                    ));
                    action_step_task.execute()?
                }
                PromptStepTask(prompt_step_task) => {
                    eprintln!(
                        "{} Running Prompt Step: {}",
                        HATCHING_CHICK_EMOJI,
                        prompt_step_task.get_name().cyan().bold()
                    );

                    pb.set_message(format!(
                        "Step '{}' in progress...",
                        prompt_step_task.get_name().bold()
                    ));
                    prompt_step_task.execute()?
                }
            };

            pb.inc(1);
            pb.tick();
            while next_step != Step::Stop {
                if let Step::Start(_) = next_step {
                    return Err(Box::new(BGitError::new(
                        "next_step must not be a Start Task!",
                        "next_step must not be a Start Task! This is a bug in the code",
                        "WorkflowQueue",
                        NO_STEP,
                        NO_EVENT,
                        NO_RULE,
                    )));
                }

                if let Step::Task(task) = next_step {
                    next_step = match task {
                        ActionStepTask(action_step_task) => {
                            eprintln!(
                                "{} Running Action Step: {}",
                                HATCHING_CHICK_EMOJI,
                                action_step_task.get_name().cyan().bold()
                            );

                            pb.set_message(format!(
                                "Step '{}' in progress...",
                                action_step_task.get_name().bold()
                            ));
                            action_step_task.execute()?
                        }
                        PromptStepTask(prompt_step_task) => {
                            eprintln!(
                                "{} Running Prompt Task: {}",
                                HATCHING_CHICK_EMOJI,
                                prompt_step_task.get_name().cyan().bold()
                            );

                            pb.set_message(format!(
                                "Step '{}' in progress...",
                                prompt_step_task.get_name().bold()
                            ));
                            prompt_step_task.execute()?
                        }
                    }
                } else {
                    unreachable!("This code is unreachable")
                }

                pb.inc(1);
                pb.tick();
            }

            pb.finish_with_message("Workflow complete");

            if next_step == Step::Stop {
                println!("Done in {}", HumanDuration(started.elapsed()));
                Ok(true)
            } else {
                Err(Box::new(BGitError::new(
                    "final_step must be a Stop Task!",
                    "final_step must be a Stop Task! This is a bug in the code",
                    "WorkflowQueue",
                    NO_STEP,
                    NO_EVENT,
                    NO_RULE,
                )))
            }
        } else {
            Err(Box::new(BGitError::new(
                "init_step must be a Start Task!",
                "init_step must be a Start Task! This is a bug in the code",
                "WorkflowQueue",
                NO_STEP,
                NO_EVENT,
                NO_RULE,
            )))
        }
    }
}
