pub struct LocalPlayer {
    name: String,
}

impl LocalPlayer {
    pub fn new(name: String) -> LocalPlayer {
        LocalPlayer { name }
    }
}
