use bevy::prelude::*;

use crate::lib::pathfinding::PathNode;
use crate::lib::space::I3;
use crate::lib::unit::Stance;

// TODO: separate components
#[derive(Default)]
pub struct Dwarf {
    pub speed: f32,
    pub p_target: Vec3,
    pub path: Option<(Vec<PathNode>, i32)>,
    pub stance: Stance,
}

// TODO: pathfinding
impl Dwarf {
    pub fn path_node(&self) -> PathNode {
        PathNode::new(self.stance, I3::from_vec(self.p_target))
    }
}
