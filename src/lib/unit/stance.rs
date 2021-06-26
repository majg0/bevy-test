use crate::lib::space::Direction;
use crate::lib::space::Orientation;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
pub enum Stance {
    Standing,
    Climbing(Orientation),
    Swimming,
    Diving {
        // breath: f32,
    },
    Falling {
        // velocity: Vec3,
    },
}

impl Stance {
    pub fn start_climb(forward: Direction) -> Stance {
        Stance::Climbing(Orientation::new_upright(forward))
    }
}

impl Default for Stance {
    fn default() -> Stance {
        Stance::Standing
    }
}
