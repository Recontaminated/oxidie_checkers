use std::collections::HashMap;

#[derive(Debug)]
pub struct CachedValue {
    pub value: i32,
    pub depth: u8,
    pub flag: flag,

}
#[derive(Debug, PartialEq)]
pub enum flag { //https://stackoverflow.com/questions/63649594/alpha-beta-pruning-with-transposition-tables
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
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let random_bitstrings: Vec<u64> = (0..256).map(|_| rng.gen::<u64>()).collect();

        TranspositionTable {
            random_bitstrings,
            map: HashMap::new(),
            cache_hits: 0,
        }
    }

    pub fn put(&mut self, bitboard: &CheckersBitboard, cached_value: CachedValue) {
        let hash = self.calculate_hash(bitboard);
        self.map.insert(hash, cached_value);
    }

    pub fn get(&mut self, bitboard: &CheckersBitboard) -> Option<&CachedValue> {
        let hash = self.calculate_hash(bitboard);
        if self.map.contains_key(&hash) {
            self.cache_hits += 1;
        }
        self.map.get(&hash)
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

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
                    hash ^= self.random_bitstrings[64 * board_idx + bit_idx];
                }
            }
        }

        hash
    }
} 
