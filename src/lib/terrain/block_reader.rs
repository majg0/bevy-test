use bevy::prelude::Query;

use crate::lib::space::I3;
use crate::lib::terrain::Block;
use crate::lib::terrain::Chunk;
use crate::lib::terrain::Map;
use crate::lib::terrain::StateOfMatter;

pub struct BlockReader<'a, 'b, 'c> {
    pub map: &'a Map,
    pub query: &'a Query<'b, &'c Chunk>,
}

impl<'a, 'b, 'c> BlockReader<'a, 'b, 'c> {
    pub fn get(&self, p: I3) -> Option<Block> {
        if let Some(e) = self.map.get_chunk_at_block(p) {
            if let Ok(c) = self.query.get(e) {
                return Some(c[p]);
            }
        }
        None
    }

    pub fn gas(&self, p: I3) -> bool {
        if let Some(e) = self.map.get_chunk_at_block(p) {
            if let Ok(c) = self.query.get(e) {
                return c[p].state_of_matter() == StateOfMatter::Gas;
            }
        }
        false
    }

    pub fn solid(&self, p: I3) -> bool {
        if let Some(e) = self.map.get_chunk_at_block(p) {
            if let Ok(c) = self.query.get(e) {
                return c[p].state_of_matter() == StateOfMatter::Solid;
            }
        }
        false
    }
}
