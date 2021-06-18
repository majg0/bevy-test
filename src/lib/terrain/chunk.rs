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

pub struct Chunk {
    pub blocks: [Block; CHUNK_VOLUME],
}

impl Chunk {
    pub fn i_to_xyz(i: usize) -> (usize, usize, usize) {
        // TODO different pattern... 4x4x4 first, then next 4x4x4 etc
        (
            i & CHUNK_SIDE_SUB1,
            (i >> CHUNK_SIDE_POW2) & CHUNK_SIDE_SUB1,
            (i >> CHUNK_SIDE_POW) & CHUNK_SIDE_SUB1,
        )
    }
    pub fn xyz_to_i(x: usize, y: usize, z: usize) -> usize {
        // TODO different pattern... 4x4x4 first, then next 4x4x4 etc
        (y << CHUNK_SIDE_POW2) | (z << CHUNK_SIDE_POW) | x
    }
    pub fn i3_to_i(p: I3) -> usize {
        Chunk::xyz_to_i(p.x as usize, p.y as usize, p.z as usize)
    }
    // pub unsafe fn at_local_raw(&self, p: I3) -> Block {
    //     self.blocks[Chunk::xyz_to_i(p.x as usize, p.y as usize, p.z as usize)]
    // }
    pub fn at_local(&self, p: I3) -> Option<Block> {
        if p.x < 0 || p.y < 0 || p.z < 0 {
            return None;
        }
        let i = Chunk::i3_to_i(p);
        if i < CHUNK_VOLUME {
            Some(self.blocks[i])
        } else {
            None
        }
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            blocks: [Block::Air; CHUNK_VOLUME],
        }
    }
}
