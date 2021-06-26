use crate::lib::terrain::StateOfMatter;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Block {
    Air,
    Dirt,
}

impl Block {
    pub fn state_of_matter(self) -> StateOfMatter {
        match self {
            Block::Air => StateOfMatter::Gas,
            Block::Dirt => StateOfMatter::Solid,
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Block::Air
    }
}
