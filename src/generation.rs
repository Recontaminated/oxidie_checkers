/*
    This is the module that handles the move table
 */

//allos us to statically have one instance of the lookup table
use once_cell::sync::Lazy;

// oop equivalent of instance vars sort of
// holds all the moves for each piece
pub struct LookupMoves {
    pub all_non_capturing_moves: [[u64; 64];3],// white black, king
    pub all_capturing_moves: [[u64; 64];3],
}

pub static LOOKUP_TABLE: Lazy<LookupMoves> = Lazy::new(|| {


    let mut moves = [[0u64; 64]; 3];
    let mut captures = [[0u64; 64]; 3]; 
        for i in 0..64 {
            let position:u64 = 1u64 << 63- i;
            //if we arent on the top row, or the left column or right
            if (i % 8 != 0) && (i > 7) && (i % 8 != 7) {
                moves[0][i] |= position << 7;
                moves[0][i] |= position << 9;
 
            }
            // if we are in left col but not top row
            else if (i % 8 == 0) && (i > 7) {
                moves[0][i] |= position << 7;
            
            }
            // if we are in right col but not top row
            else if (i % 8 == 7) && (i > 7) {
                moves[0][i] |= position << 9;
                captures[0][i] |= position << 18;
            }
        
            // if we arent on the bottom row or left or right column
            if (i < 56) && (i % 8 != 0) && (i % 8 != 7) {
                moves[1][i] |= position >> 7;
                moves[1][i] |= position >> 9;
              
                
            }
            // if we are in left col but not bottom row
            else if (i % 8 == 0) && (i < 56) {
                moves[1][i] |= position >> 9;
         
            }
            // if we are in right col but not bottom row
            else if (i % 8 == 7) && (i < 56) {
                moves[1][i] |= position >> 7;
             
            }

            // figrue out captures
            // if we aren in the top 2 rows and not in the left 2 cols or right 2 cols
            if (i > 15) && (i % 8 > 1) && (i % 8 < 6) {
                captures[0][i] |= position << 14;
                captures[0][i] |= position << 18;
            }
            // if we arent in bottom 2 rows and not in left 2 cols or right 2 cols
            if (i < 48) && (i % 8 > 1) && (i % 8 < 6) {
                captures[1][i] |= position >> 14;
                captures[1][i] |= position >> 18;
            }
            //if we are in left 2 cols and not in top 2 rows
            if (i % 8 < 2) && (i > 15) {
                captures[0][i] |= position << 14;
            }
            //if we are in right two rows and not in top 2 rows
            if (i % 8 > 5) && (i > 15) {
                captures[0][i] |= position << 18;
            }
            //if we are in left 2 cols and not in bottom 2 rows
            if (i % 8 < 2) && (i < 48) {
                captures[1][i] |= position >> 18;
            }
            //if we are in right 2 cols and not in bottom 2 rows
            if (i % 8 > 5) && (i < 48) {
                captures[1][i] |= position >> 14;
            }





            moves[2][i] = moves[0][i] | moves[1][i];
            captures[2][i] = captures[0][i] | captures[1][i];

        }
    let all_non_capturing_moves = moves;
    let all_capturing_moves = captures;

    LookupMoves { all_non_capturing_moves , all_capturing_moves}
});


