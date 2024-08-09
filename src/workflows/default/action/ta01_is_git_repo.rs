use crate::tasks::Task;

use super::BgitTask;

struct ta01_is_git_repo {}

impl BgitTask for ta01_is_git_repo {
    fn new(name: String, id: u32) -> Self where Self: Sized {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }

    fn get_id(&self) -> u32 {
        todo!()
    }

    fn apply_task(&self) -> Result<Option<Box<dyn BgitTask>>, &str> {
        todo!()
    }
}
