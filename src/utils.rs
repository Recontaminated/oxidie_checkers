pub fn pretty_print_bitboard(bitboard: u64) {
    let bitboard = bitboard;
    for row in 0..8 {
        for col in 0..8 {
            print!(
                "{}",
                if bitboard & 1u64 << 63 - (row * 8 + col) != 0 {
                    "1"
                } else {
                    "0"
                }
            );
        }
        println!();
    }
}

pub fn lsb_idx (bitboard: &u64) -> u8 {
    63 - bitboard.trailing_zeros() as u8 // we do this because otherwise bottom right of our board is 0 and top left is 63
}