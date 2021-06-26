#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
pub enum PlayerState {
    Select,
    RemoveBlock,
}

impl Default for PlayerState {
    fn default() -> PlayerState {
        PlayerState::Select
    }
}
