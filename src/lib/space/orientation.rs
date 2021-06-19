use crate::lib::space::Direction;
use crate::lib::space::I3;

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct Orientation {
    pub forward: Direction,
    pub up: Direction,
    pub right: Direction,
}

impl Orientation {
    pub fn new_upright(forward: Direction) -> Orientation {
        let up = Direction::Yp;
        Orientation {
            forward,
            up,
            right: up.cross(forward),
        }
    }
    pub fn adjust_i3(self, v: I3) -> I3 {
        v.z * self.forward.i3() + v.y * self.up.i3() + v.x * self.right.i3()
    }
}
