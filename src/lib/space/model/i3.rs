use bevy::prelude::Vec3;
use std::ops;

use crate::lib::space::Direction;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct I3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl I3 {
    // Constructors
    pub fn new(x: i32, y: i32, z: i32) -> I3 {
        I3 { x, y, z }
    }
    pub fn xn() -> I3 {
        I3 { x: -1, y: 0, z: 0 }
    }
    pub fn xp() -> I3 {
        I3 { x: 1, y: 0, z: 0 }
    }
    pub fn yn() -> I3 {
        I3 { x: 0, y: -1, z: 0 }
    }
    pub fn yp() -> I3 {
        I3 { x: 0, y: 1, z: 0 }
    }
    pub fn zn() -> I3 {
        I3 { x: 0, y: 0, z: -1 }
    }
    pub fn zp() -> I3 {
        I3 { x: 0, y: 0, z: 1 }
    }
    pub fn from_vec(v: Vec3) -> I3 {
        I3 {
            x: v.x as i32,
            y: v.y as i32,
            z: v.z as i32,
        }
    }
    // Binary Operations
    pub fn cross(self, v: I3) -> I3 {
        I3::new(
            self.y * v.z - self.z * v.y,
            self.x * v.z - self.z * v.x,
            self.x * v.y - self.y * v.x,
        )
    }
    pub fn map<F: Fn(i32) -> i32>(self, f: F) -> I3 {
        I3::new(f(self.x), f(self.y), f(self.z))
    }
    pub fn each_less_than(self, v: I3) -> bool {
        self.x < v.x && self.y < v.y && self.z < v.z
    }
    // Conversions
    pub fn vec3(self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }
    pub fn direction(self) -> Option<Direction> {
        match self {
            I3 { x: -1, y: 0, z: 0 } => Some(Direction::Xn),
            I3 { x: 1, y: 0, z: 0 } => Some(Direction::Xp),
            I3 { x: 0, y: -1, z: 0 } => Some(Direction::Yn),
            I3 { x: 0, y: 1, z: 0 } => Some(Direction::Yp),
            I3 { x: 0, y: 0, z: -1 } => Some(Direction::Zn),
            I3 { x: 0, y: 0, z: 1 } => Some(Direction::Zp),
            _ => None,
        }
    }
    // Transformations
    pub fn px(self) -> I3 {
        I3 {
            x: self.x + 1,
            ..self
        }
    }
    pub fn px_py(self) -> I3 {
        I3 {
            x: self.x + 1,
            y: self.y + 1,
            ..self
        }
    }
    pub fn px_py_pz(self) -> I3 {
        I3 {
            x: self.x + 1,
            y: self.y + 1,
            z: self.z + 1,
        }
    }
    pub fn px_py_nz(self) -> I3 {
        I3 {
            x: self.x + 1,
            y: self.y + 1,
            z: self.z - 1,
        }
    }
    pub fn px_ny(self) -> I3 {
        I3 {
            x: self.x + 1,
            y: self.y - 1,
            ..self
        }
    }
    pub fn px_ny_pz(self) -> I3 {
        I3 {
            x: self.x + 1,
            y: self.y - 1,
            z: self.z + 1,
        }
    }
    pub fn px_ny_nz(self) -> I3 {
        I3 {
            x: self.x + 1,
            y: self.y - 1,
            z: self.z - 1,
        }
    }
    pub fn px_pz(self) -> I3 {
        I3 {
            x: self.x + 1,
            z: self.z + 1,
            ..self
        }
    }
    pub fn px_nz(self) -> I3 {
        I3 {
            x: self.x + 1,
            z: self.z - 1,
            ..self
        }
    }

    pub fn nx(self) -> I3 {
        I3 {
            x: self.x - 1,
            ..self
        }
    }
    pub fn nx_py(self) -> I3 {
        I3 {
            x: self.x - 1,
            y: self.y + 1,
            ..self
        }
    }
    pub fn nx_py_pz(self) -> I3 {
        I3 {
            x: self.x - 1,
            y: self.y + 1,
            z: self.z + 1,
        }
    }
    pub fn nx_py_nz(self) -> I3 {
        I3 {
            x: self.x - 1,
            y: self.y + 1,
            z: self.z - 1,
        }
    }
    pub fn nx_ny(self) -> I3 {
        I3 {
            x: self.x - 1,
            y: self.y - 1,
            ..self
        }
    }
    pub fn nx_ny_pz(self) -> I3 {
        I3 {
            x: self.x - 1,
            y: self.y - 1,
            z: self.z + 1,
        }
    }
    pub fn nx_ny_nz(self) -> I3 {
        I3 {
            x: self.x - 1,
            y: self.y - 1,
            z: self.z - 1,
        }
    }
    pub fn nx_pz(self) -> I3 {
        I3 {
            x: self.x - 1,
            z: self.z + 1,
            ..self
        }
    }
    pub fn nx_nz(self) -> I3 {
        I3 {
            x: self.x - 1,
            z: self.z - 1,
            ..self
        }
    }

    pub fn py(self) -> I3 {
        I3 {
            y: self.y + 1,
            ..self
        }
    }
    pub fn py_pz(self) -> I3 {
        I3 {
            y: self.y + 1,
            z: self.z + 1,
            ..self
        }
    }
    pub fn py_nz(self) -> I3 {
        I3 {
            y: self.y + 1,
            z: self.z - 1,
            ..self
        }
    }

    pub fn ny(self) -> I3 {
        I3 {
            y: self.y - 1,
            ..self
        }
    }
    pub fn ny_pz(self) -> I3 {
        I3 {
            y: self.y - 1,
            z: self.z + 1,
            ..self
        }
    }
    pub fn ny_nz(self) -> I3 {
        I3 {
            y: self.y - 1,
            z: self.z - 1,
            ..self
        }
    }

    pub fn pz(self) -> I3 {
        I3 {
            z: self.z + 1,
            ..self
        }
    }
    pub fn nz(self) -> I3 {
        I3 {
            z: self.z - 1,
            ..self
        }
    }
}

impl ops::Add<I3> for I3 {
    type Output = I3;

    fn add(self, rhs: I3) -> I3 {
        I3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Mul<i32> for I3 {
    type Output = I3;

    fn mul(self, rhs: i32) -> I3 {
        I3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<I3> for i32 {
    type Output = I3;

    fn mul(self, rhs: I3) -> I3 {
        I3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}
