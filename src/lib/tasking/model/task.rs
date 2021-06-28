use crate::lib::space::I3;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Task {
    RemoveBlock(RemoveBlockTask),
}

impl Task {
    pub fn remove_block(position: I3) -> Task {
        Task::RemoveBlock(RemoveBlockTask { position })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct RemoveBlockTask {
    pub position: I3,
}
