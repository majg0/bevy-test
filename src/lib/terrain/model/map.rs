use bevy::prelude::Entity;

use crate::lib::space::I3;
use crate::lib::terrain::CHUNK_SIDE_POW;
use crate::lib::terrain::CHUNK_SIDE_SUB1_I;

pub struct Map {
    pub size_in_blocks: I3, // TODO U3
    pub size_in_chunks: I3, // TODO U3
    pub chunks: Vec<Entity>,
}

fn num_chunks(n: i32) -> i32 {
    let q = n >> CHUNK_SIDE_POW;
    let r = if (n & CHUNK_SIDE_SUB1_I) != 0 { 1 } else { 0 };
    q + r
}

impl Map {
    pub fn new(size_in_blocks: I3) -> Map {
        let chunks = Vec::new();
        let size_in_chunks = size_in_blocks.map(num_chunks);
        Map {
            size_in_blocks,
            size_in_chunks,
            chunks,
        }
    }
    pub fn i_to_p(&self, i: usize) -> I3 {
        let mut x = i as i32;
        let size_xz = self.size_in_chunks.z * self.size_in_chunks.x;
        let size_x = self.size_in_chunks.x;
        let y = x / size_xz;
        x -= y * size_xz;
        let z = x / size_x;
        x -= z * size_x;
        I3::new(x, y, z)
    }
    pub fn p_to_i(&self, p: I3) -> usize {
        (p.y * (self.size_in_chunks.z * self.size_in_chunks.x) + p.z * self.size_in_chunks.x + p.x)
            as usize
    }
    pub fn num_chunks(&self) -> usize {
        (self.size_in_chunks.x * self.size_in_chunks.y * self.size_in_chunks.z) as usize
    }
    pub fn get_chunk_at_block(&self, p: I3) -> Option<Entity> {
        if p.x < 0
            || p.y < 0
            || p.z < 0
            || p.x >= self.size_in_blocks.x
            || p.y >= self.size_in_blocks.y
            || p.z >= self.size_in_blocks.z
        {
            return None;
        }
        Some(self.chunks[self.p_to_i(p.map(|i| i >> CHUNK_SIDE_POW))])
    }
}
