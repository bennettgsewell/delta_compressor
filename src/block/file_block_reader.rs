#[cfg(test)]
mod tests {
    use super::FileBlockReader;

    #[test]
    #[should_panic(expected = "block_size cannot be zero!")]
    pub fn new_when_block_size_zero_should_panic() {
        FileBlockReader::new("", 0).expect("This should not occur");
    }
}

use super::{block_reader::BlockReader, Block, BlockID};
use crate::constants::binary_prefixes::*;
use std::{
    fs::File,
    io::{self, Read},
    os::unix::fs::FileExt,
};

const CHUNK_BUF_SIZE: usize = 8 * KB;

/// Reads a raw file in chunks of data called Blocks.
pub struct FileBlockReader {
    /// File being read
    f: File,

    /// The size of a Block
    block_size: usize,
    /// The currently selected Block
    current_block_id: BlockID,

    /// Byte location of the currently loaded chunk
    chunk_offset: usize,
    /// Current length of good data in the chunk_buf
    chunk_length: usize,
    /// How many single bytes have been read from the chunk_buf into a Block so far.
    /// When this equals the chunk length, attempt to load more data into the chunk
    /// from the file.
    chunk_digested: usize,
    /// File data is read into this buffer, then it's broken into Blocks as required
    chunk_buf: Box<[u8; CHUNK_BUF_SIZE]>,

    /// Has the entire file been read.
    entire_file_read: bool,
}

impl FileBlockReader {
    /// Opens a file from its path, returns None on failure.
    pub fn new(path: &str, block_size: usize) -> io::Result<FileBlockReader> {
        // Safety check
        if block_size == 0 {
            panic!("block_size cannot be zero!");
        }

        Ok(FileBlockReader {
            // Attmept to open the file, Err on failure
            f: File::open(path)?,
            block_size,
            current_block_id: 0,
            chunk_offset: 0,
            chunk_length: 0,
            chunk_digested: 0,
            chunk_buf: Box::new([0u8; CHUNK_BUF_SIZE]),
            entire_file_read: false,
        })
    }

    /// Reads the next byte of the file, if the buffer runs out
    /// it reads the next "chunk" of data
    pub fn read_byte(&mut self) -> io::Result<Option<u8>> {
        // The entire file has already been read, no more data to go through.
        if self.entire_file_read {
            return Ok(None);
        }

        // All the bytes in the chunk_buf have been read
        // Load the next chunk of data
        if self.chunk_digested == self.chunk_length {
            // Move the offset forward
            self.chunk_offset += CHUNK_BUF_SIZE;

            // Read the next chunk into the buffer
            self.chunk_length = self
                .f
                .read_at(&mut (*self.chunk_buf), self.chunk_offset as u64)?;

            // None of the new chunk has been digested yet.
            self.chunk_digested = 0;

            // The entire file has been read, nothing more to read.
            if self.chunk_length == 0 {
                self.entire_file_read = true;
                return Ok(None);
            }
        }

        // Read the next byte in the chunk
        match self.chunk_buf.get(self.chunk_digested) {
            None => Ok(None),
            Some(data) => Ok(Some(*data)),
        }
    }
}

impl BlockReader for FileBlockReader {
    fn get_current_block_number(&self) -> Option<BlockID> {
        Some(self.current_block_id)
    }

    fn read(&mut self) -> Option<Block> {
        let mut buffer = Vec::<u8>::new();
        // Preallocate the buffer to the correct size
        buffer.reserve(self.block_size);

        for i in 0..self.block_size {
            match self.read_byte().expect("Failed to read data from file!") {
                Some(more_data) => buffer.push(more_data),
                None => break,
            }
        }

        match buffer.len() {
            0 => None,
            _ => {
                // Create the Block
                let result = Some(Block {
                    block_number: self.current_block_id,
                    buf: buffer,
                });

                // Increment the block
                self.current_block_id += 1;

                result
            }
        }
    }
}
