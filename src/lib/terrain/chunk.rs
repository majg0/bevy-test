use std::ops::Index;

use crate::lib::space::I3;
use crate::lib::terrain::Block;

pub const CHUNK_SIDE_POW: usize = 4;
pub const CHUNK_SIDE_POW2: usize = CHUNK_SIDE_POW * 2;
pub const CHUNK_SIDE: usize = 1 << CHUNK_SIDE_POW;
pub const CHUNK_SIDE_F: f32 = CHUNK_SIDE as f32;
// pub const CHUNK_SIDE_I: i32 = CHUNK_SIDE as i32;
pub const CHUNK_AREA: usize = CHUNK_SIDE * CHUNK_SIDE;
pub const CHUNK_VOLUME: usize = CHUNK_AREA * CHUNK_SIDE;
pub const CHUNK_SIDE_SUB1: usize = CHUNK_SIDE - 1;
pub const CHUNK_SIDE_SUB1_I: i32 = CHUNK_SIDE_SUB1 as i32;

pub struct Chunk {
    pub blocks: [Block; CHUNK_VOLUME],
}

impl Chunk {
    pub fn i_to_p(i: usize) -> I3 {
        // TODO different pattern... 4x4x4 first, then next 4x4x4 etc
        I3::new(
            (i & CHUNK_SIDE_SUB1) as i32,
            ((i >> CHUNK_SIDE_POW2) & CHUNK_SIDE_SUB1) as i32,
            ((i >> CHUNK_SIDE_POW) & CHUNK_SIDE_SUB1) as i32,
        )
    }
    pub fn xyz_to_i(x: usize, y: usize, z: usize) -> usize {
        // TODO different pattern... 4x4x4 first, then next 4x4x4 etc
        (y << CHUNK_SIDE_POW2) | (z << CHUNK_SIDE_POW) | x
    }
    pub fn i3_to_i(p: I3) -> usize {
        Chunk::xyz_to_i(
            (p.x as usize) & CHUNK_SIDE_SUB1,
            (p.y as usize) & CHUNK_SIDE_SUB1,
            (p.z as usize) & CHUNK_SIDE_SUB1,
        )
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            blocks: [Block::Air; CHUNK_VOLUME],
        }
    }
}

impl Index<I3> for Chunk {
    type Output = Block;

    fn index(&self, p: I3) -> &Self::Output {
        &self.blocks[Chunk::i3_to_i(p)]
    }
}
