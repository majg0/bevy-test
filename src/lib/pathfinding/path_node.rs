use bevy::prelude::Query;
use pathfinding::prelude::astar;

use crate::lib::space::Direction;
use crate::lib::space::I3;
use crate::lib::terrain::Block;
use crate::lib::terrain::BlockReader;
use crate::lib::terrain::Chunk;
use crate::lib::terrain::Map;
use crate::lib::terrain::StateOfMatter;
use crate::lib::unit::Stance;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
pub struct PathNode {
    pub stance: Option<Stance>,
    pub p: I3,
}

impl PathNode {
    pub fn new(stance: Stance, p: I3) -> PathNode {
        PathNode {
            stance: Some(stance),
            p,
        }
    }

    pub fn path_to(
        self: PathNode,
        map: &Map,
        chunk_query: &Query<&Chunk>,
        goal: PathNode,
    ) -> Option<(Vec<PathNode>, i32)> {
        let blocks = BlockReader {
            map,
            query: &chunk_query,
        };
        astar(
            &self,
            |node| {
                let mut result = Vec::new();

                let p0 = node.p;
                let stance = node.stance.unwrap();

                // TODO: loop through all "capabilities" (walk, dive, swim, climb, jump, etc etc) in order of preference and attempt them

                match stance {
                    Stance::Standing => {
                        for &dir in Direction::all_xz().iter() {
                            let p = p0 + dir.i3();
                            let p_d = p.ny();
                            let s_p = blocks.get(p).map(Block::state_of_matter);
                            let s_d = blocks.get(p_d).map(Block::state_of_matter);
                            if s_p == None {
                                continue;
                            }
                            let s_p = s_p.unwrap();
                            if s_p == StateOfMatter::Gas {
                                if s_d == Some(StateOfMatter::Solid) {
                                    result.push((PathNode::new(Stance::Standing, p), 1));
                                } else if s_d == Some(StateOfMatter::Gas) {
                                    // Climb down ledge
                                    let stance = Stance::start_climb(dir.flip());
                                    result.push((PathNode::new(stance, p_d), 2));
                                }
                            } else if s_p == StateOfMatter::Solid {
                                // Climb wall
                                result.push((PathNode::new(Stance::start_climb(dir), p0), 1));
                            }
                        }
                    }
                    Stance::Climbing(orientation) => {
                        let grab_dir = orientation.forward;
                        let p0_u = p0 + orientation.u();
                        let p0_d = p0 + orientation.d();
                        let p0_r = p0 + orientation.r();
                        let p0_l = p0 + orientation.l();
                        let p = p0 + grab_dir.i3();
                        let p_u = p + orientation.u();
                        let p_d = p + orientation.d();
                        let p_r = p + orientation.r();
                        let p_l = p + orientation.l();

                        // Turn in tile
                        if blocks.solid(p0_r) {
                            result.push((
                                PathNode::new(Stance::Climbing(orientation.yaw_right()), p0),
                                1,
                            ));
                        }
                        if blocks.solid(p0_l) {
                            result.push((
                                PathNode::new(Stance::Climbing(orientation.yaw_left()), p0),
                                1,
                            ));
                        }

                        // Mount ledge
                        if blocks.gas(p0_u) && blocks.gas(p_u) {
                            result.push((PathNode::new(Stance::Standing, p_u), 2));
                        }

                        // Drop down
                        if blocks.solid(p0_d) {
                            result.push((PathNode::new(Stance::Standing, p0), 1));
                        }

                        // Climb along wall
                        if blocks.gas(p0_u) && blocks.solid(p_u) {
                            result.push((PathNode::new(stance, p0_u), 1));
                        }
                        if blocks.gas(p0_d) && blocks.solid(p_d) {
                            result.push((PathNode::new(stance, p0_d), 1));
                        }
                        if blocks.gas(p0_l) && blocks.solid(p_l) {
                            result.push((PathNode::new(stance, p0_l), 1));
                        }
                        if blocks.gas(p0_r) && blocks.solid(p_r) {
                            result.push((PathNode::new(stance, p0_r), 1));
                        }
                    }
                    _ => panic!("TODO stance"),
                }

                result
            },
            |node| {
                let p = node.p;
                p.x * p.x + p.y * p.y + p.z * p.z
            },
            |node| {
                let p = node.p;
                p == goal.p
            },
        )
    }
}
