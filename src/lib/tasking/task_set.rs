use crate::lib::space::I3;
use crate::lib::tasking::TaskDescription;
use crate::lib::tasking::TaskKind;

#[derive(Debug, Default)]
pub struct TaskSet {
    pub tasks: Vec<TaskDescription>,
}

impl TaskSet {
    pub fn remove_block(&mut self, pos: I3) -> &mut Self {
        self.tasks
            .push(TaskDescription::new(TaskKind::RemoveBlock, pos));
        self
    }
}
