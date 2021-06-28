use crate::lib::tasking::Task;

#[derive(Debug, Default)]
pub struct TaskProcessor {
    pub queue: Vec<Task>,
}
