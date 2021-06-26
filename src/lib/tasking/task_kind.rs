#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum TaskKind {
    Idle,
    RemoveBlock,
}
