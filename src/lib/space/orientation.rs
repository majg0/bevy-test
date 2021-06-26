use crate::lib::space::Direction;
use crate::lib::space::I3;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
pub struct Orientation {
    pub right: Direction,
    pub up: Direction,
    pub forward: Direction,
}

impl Orientation {
    // Constructors
    pub fn new_upright(forward: Direction) -> Orientation {
        let up = Direction::Yp;
        Orientation {
            right: up.cross(forward),
            up,
            forward,
        }
    }
    // Complementary Directions
    pub fn left(self) -> Direction {
        self.right.flip()
    }
    pub fn down(self) -> Direction {
        self.up.flip()
    }
    pub fn back(self) -> Direction {
        self.forward.flip()
    }
    // Miscellaneous
    pub fn adjust_i3(self, v: I3) -> I3 {
        v.z * self.forward.i3() + v.y * self.up.i3() + v.x * self.right.i3()
    }
    // Rotations
    pub fn yaw_left(self) -> Orientation {
        Orientation {
            right: self.forward,
            up: self.up,
            forward: self.right.flip(),
        }
    }
    pub fn yaw_right(self) -> Orientation {
        Orientation {
            right: self.forward.flip(),
            up: self.up,
            forward: self.right,
        }
    }
    pub fn pitch_up(self) -> Orientation {
        Orientation {
            right: self.right,
            up: self.forward.flip(),
            forward: self.up,
        }
    }
    pub fn pitch_down(self) -> Orientation {
        Orientation {
            right: self.right,
            up: self.forward,
            forward: self.up.flip(),
        }
    }
    pub fn roll_left(self) -> Orientation {
        Orientation {
            right: self.up,
            up: self.right.flip(),
            forward: self.forward,
        }
    }
    pub fn roll_right(self) -> Orientation {
        Orientation {
            right: self.up.flip(),
            up: self.right,
            forward: self.forward,
        }
    }
    // Relative I3's
    pub fn luf(self) -> I3 {
        self.left().i3() + self.up.i3() + self.forward.i3()
    }
    pub fn uf(self) -> I3 {
        self.up.i3() + self.forward.i3()
    }
    pub fn ruf(self) -> I3 {
        self.right.i3() + self.up.i3() + self.forward.i3()
    }
    pub fn lf(self) -> I3 {
        self.left().i3() + self.forward.i3()
    }
    pub fn f(self) -> I3 {
        self.forward.i3()
    }
    pub fn rf(self) -> I3 {
        self.right.i3() + self.forward.i3()
    }
    pub fn ldf(self) -> I3 {
        self.left().i3() + self.down().i3() + self.forward.i3()
    }
    pub fn df(self) -> I3 {
        self.down().i3() + self.forward.i3()
    }
    pub fn rdf(self) -> I3 {
        self.right.i3() + self.down().i3() + self.forward.i3()
    }
    pub fn lu(self) -> I3 {
        self.left().i3() + self.up.i3()
    }
    pub fn u(self) -> I3 {
        self.up.i3()
    }
    pub fn ru(self) -> I3 {
        self.right.i3() + self.up.i3()
    }
    pub fn l(self) -> I3 {
        self.left().i3()
    }
    pub fn r(self) -> I3 {
        self.right.i3()
    }
    pub fn ld(self) -> I3 {
        self.left().i3() + self.down().i3()
    }
    pub fn d(self) -> I3 {
        self.down().i3()
    }
    pub fn rd(self) -> I3 {
        self.right.i3() + self.down().i3()
    }
    pub fn lub(self) -> I3 {
        self.left().i3() + self.up.i3() + self.back().i3()
    }
    pub fn ub(self) -> I3 {
        self.up.i3() + self.back().i3()
    }
    pub fn rub(self) -> I3 {
        self.right.i3() + self.up.i3() + self.back().i3()
    }
    pub fn lb(self) -> I3 {
        self.left().i3() + self.back().i3()
    }
    pub fn b(self) -> I3 {
        self.back().i3()
    }
    pub fn rb(self) -> I3 {
        self.right.i3() + self.back().i3()
    }
    pub fn ldb(self) -> I3 {
        self.left().i3() + self.down().i3() + self.back().i3()
    }
    pub fn db(self) -> I3 {
        self.down().i3() + self.back().i3()
    }
    pub fn rdb(self) -> I3 {
        self.right.i3() + self.down().i3() + self.back().i3()
    }
}
