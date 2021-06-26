use crate::lib::space::I3;
use crate::lib::tasking::TaskKind;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
pub struct TaskDescription {
    pub task_kind: TaskKind,
    pub pos: I3,
}

impl TaskDescription {
    pub fn new(task_kind: TaskKind, pos: I3) -> TaskDescription {
        TaskDescription { task_kind, pos }
    }
}
