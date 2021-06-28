use crate::lib::space::I3;
use crate::lib::tasking::Task;

#[derive(Debug, Default)]
pub struct TaskScheduler {
    pub tasks: Vec<Task>,
}

impl TaskScheduler {
    pub fn remove_block(&mut self, position: I3) {
        self.tasks.push(Task::remove_block(position));
    }
}
