/*
    This module handles caching previously seen board states
    
 */
use std::collections::HashMap;

#[derive(Debug)]
pub struct CachedValue {
    pub value: i32,
    pub depth: u8,
    pub flag: flag,

}
// we need to store some meata data about the value see  https://stackoverflow.com/questions/63649594/alpha-beta-pruning-with-transposition-tables
#[derive(Debug, PartialEq)]
pub enum flag {
    EXACT,
    LOWERBOUND,
    UPPERBOUND,
}

use rand::Rng;

use crate::board::CheckersBitboard; 




pub struct TranspositionTable {
    random_bitstrings: Vec<u64>,
    map: HashMap<u64, CachedValue>,
    pub cache_hits: u64,
}

impl TranspositionTable {
    // constructor 
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let random_bitstrings: Vec<u64> = (0..256).map(|_| rng.gen::<u64>()).collect();

        TranspositionTable {
            random_bitstrings,
            map: HashMap::new(),
            cache_hits: 0,
        }
    }

    // adds a cached entry to the table. 
    pub fn put(&mut self, bitboard: &CheckersBitboard, cached_value: CachedValue) {
        let hash = self.calculate_hash(bitboard);
        self.map.insert(hash, cached_value);
    }

    // gets a cached entry from the table or None if nothign found
    pub fn get(&mut self, bitboard: &CheckersBitboard) -> Option<&CachedValue> {
        let hash = self.calculate_hash(bitboard);
        if self.map.contains_key(&hash) {
            self.cache_hits += 1;
        }
        self.map.get(&hash)
    }
    // nuke the table. not used but added for completeness
    pub fn clear(&mut self) {
        self.map.clear();
    }

    // uses modified zorbist hash to calculate a hash for the board
    // we cram all hases into 256 so 0..63 is white pieces, 64..127 is white kings, 128..191 is black pieces, 192..255 is black kings
    fn calculate_hash(&self, bitboard: &CheckersBitboard) -> u64 {
        let mut hash = 0u64;

        let boards: [u64; 4] = [
            bitboard.white_pieces,
            bitboard.white_kings,
            bitboard.black_pieces,
            bitboard.black_kings,
        ];

        for (board_idx, board) in boards.iter().enumerate() {
            for bit_idx in 0..64 {
                if (board & (1u64 << (63 - bit_idx))) != 0 {
                    // each hash gets 64 spaces in random_bitstrings so we need to jump to its allocated space
                    hash ^= self.random_bitstrings[64 * board_idx + bit_idx];
                }
            }
        }

        hash
    }
} 
