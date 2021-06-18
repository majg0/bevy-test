use crate::lib::space::Direction;
use crate::lib::space::I3;

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct Orientation {
    pub forward: Direction,
    pub up: Direction,
    // TODO: store `right` Direction
}

impl Orientation {
    pub fn new(forward: Direction, up: Direction) -> Orientation {
        Orientation { forward, up }
    }
    pub fn right(self) -> I3 {
        let y = self.up.i3();
        let z = self.forward.i3();
        y.cross(z)
    }
    pub fn adjust_i3(self, v: I3) -> I3 {
        v.z * self.forward.i3() + v.y * self.up.i3() + v.x * self.right()
    }
}
