use crate::lib::space::I3;

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub enum Direction {
    Xn,
    Xp,
    Yn,
    Yp,
    Zn,
    Zp,
}

impl Direction {
    pub fn all_xz() -> [Direction; 4] {
        [Direction::Xn, Direction::Xp, Direction::Zn, Direction::Zp]
    }
    pub fn i3(self) -> I3 {
        match self {
            Direction::Xn => I3::xn(),
            Direction::Xp => I3::xp(),
            Direction::Yn => I3::yn(),
            Direction::Yp => I3::yp(),
            Direction::Zn => I3::zn(),
            Direction::Zp => I3::zp(),
        }
    }
    pub fn cross(self, rhs: Direction) -> Direction {
        self.i3().cross(rhs.i3()).direction().unwrap()
    }
}
