use super::{Block, BlockID};

pub trait BlockReader {
    /// The current block ID.
    fn get_current_block_number(&self) -> Option<BlockID>;

    /// Attempts to read the next block.
    fn read(&mut self) -> Option<Block>;
}