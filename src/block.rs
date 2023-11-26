#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn is_match_when_data_matches_return_true() {
        // Arrange
        let a = Block {
            block_number: 45,
            buf: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        };
        let b = Block {
            block_number: 45,
            buf: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        };

        // Act
        let result = a.is_match(&b);

        // Assert
        assert!(result);
    }

    #[test]
    pub fn is_match_when_data_dont_match_return_false() {
        // Arrange
        let a = Block {
            block_number: 45,
            buf: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        };
        let b = Block {
            block_number: 45,
            buf: vec![0, 1, 2, 3, 4, 60, 6, 7, 8],
        };

        // Act
        let result = a.is_match(&b);

        // Assert
        assert!(!result);
    }

    #[test]
    pub fn is_same_size_when_match_return_true() {
        // Arrange
        let a = Block {
            block_number: 0,
            buf: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        };

        let b = Block {
            block_number: 0,
            buf: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        };

        // Act
        let result = a.is_same_size(&b);

        // Assert
        assert!(result);
    }

    #[test]
    pub fn is_same_size_when_nomatch_return_false() {
        // Arrange
        let a = Block {
            block_number: 0,
            buf: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        };

        let b = Block {
            block_number: 0,
            buf: vec![0, 1, 2, 3, 4],
        };

        // Act
        let result = a.is_same_size(&b);

        // Assert
        assert!(!result);
    }
}

/// Files are split into blocks this represents on of those data chunks.
pub struct Block {
    /// The number of the block in the file.
    block_number: usize,

    /// The buffer of data inside the block.
    buf: Vec<u8>,
}

impl Block {
    /// Returns true if the data in the two blocks match each other.
    pub fn is_match(&self, other: &Block) -> bool {
        // Make sure the block sizes match.
        if !self.is_same_block_num(other) || !self.is_same_size(other) {
            return false;
        }

        let block_size = self.buf.len();

        for i in 0..block_size {
            // Since both of these arrays have the same length
            let a = self.buf.get(i);
            let b = other.buf.get(i);

            if a != b {
                return false;
            }
        }

        true
    }

    /// Returns true if the block_numbers match
    pub fn is_same_block_num(&self, other: &Block) -> bool {
        self.block_number == other.block_number
    }

    /// validates that the buffer in the block is the same length as the block_size
    pub fn is_same_size(&self, other: &Block) -> bool {
        self.buf.len() == other.buf.len()
    }
}
