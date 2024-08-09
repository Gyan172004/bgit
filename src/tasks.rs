mod ta01_is_git_repo;
mod ta02_ask_init_git_repo;
mod ta03_do_init_git_repo;
mod ta04_has_stash;
mod ta05_ask_pop_stash;
mod ta06_do_pop_stash;
mod ta07_has_unstaged_files;
mod ta08_ask_to_stage_file;
mod ta09_ask_to_stage_file_all_file;
mod ta10_ask_to_stage_file_discretely_add;

pub enum Task {
    ActionTask,
    InputTask
}

pub trait BgitTask {
    fn new(name: String, id: u32) -> Self where Self: Sized;
    fn get_name(&self) -> String;
    fn get_id(&self) -> u32;
    fn apply_task(&self) -> Result<Option<Box<dyn BgitTask>>, &str>;
}
