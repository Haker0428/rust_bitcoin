use crate::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn add_block(&mut self, data: String) {
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let new_block = Block::new_block(data, pre_block.hash.clone());
        self.blocks.push(new_block);
    }

    fn new_genesis_block() -> Block {
        Block::new_block("This is genesis block".to_string(), "".to_string())
    }

    pub fn new_blockchain() -> Blockchain {
        Blockchain {
            blocks: vec![Blockchain::new_genesis_block()]
        }
    }
}
