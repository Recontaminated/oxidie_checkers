/*
    nice utility functions used for misc purposes
 */

/*
    Debugging function to print a bitboard in a nice format similar to print_board
 */
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
// returns the index of the least significant bit so 00000100 would return 6
pub fn lsb_idx (bitboard: &u64) -> u8 {
    63 - bitboard.trailing_zeros() as u8 // we do this because otherwise bottom right of our board is 0 and top left is 63
}

/*
    converts a bitboard index to a coordinate string
    ex:63 -> "h1"
 */
pub fn letter_coord_to_index(coord: &str) -> Result<u8, &'static str>{
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    if coord.len() != 2 {
        return Err("Invalid coordinate length");
    }
    //input validation because we do alot of string parsing
    let first_letter = match coord.chars().nth(0){
        Some(letter) => {
            //get index of letter
            match letters.iter().position(|&r| r == letter) {
                Some(index) => index,
                None => return Err("first letter not letter"),
            }
        }
        None => return Err("Invalid coordinate"),
    };
   
    let first_number = match coord.chars().nth(1){
        Some(letter) => match letter.to_digit(10) {
            Some(num) => num,
            None => return Err("Invalid coordinate format (number not number)"),
        },
        None => return Err("Invalid coordinate length"),
    };

    if first_letter > 8 || first_number > 8 {
        return Err("Invalid coordinate (out of bounds))");
    }
    //we dont want bottom right of our board is 0 and top left is 63 we want the opposite
    Ok((first_letter) as u8 + ((8-(first_number)) as u8 * 8))




    
    
}