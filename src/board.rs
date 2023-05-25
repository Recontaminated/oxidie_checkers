use std::fs;
pub struct GameState {
    pub game: CheckersBitboard,
    pub turnWhite: bool,
}
impl GameState {
    pub fn startingPos() -> GameState {
        // open file
        // let file = fs::read_to_string("src/startState.txt").expect("Unable to read file");
        const startingString:&str = "1b1b1b1b/b1b1b1b1/1b1b1b1b/8/8/w1w1w1w1/1w1w1w1w/w1w1w1w1 w";
        let game = CheckersBitboard::new();
        let mut state = GameState {
            game: game,
            turnWhite: true,
        };
        state.loadFromString(startingString);
        state
    }
    pub fn movePiece(&mut self, from: u8, to: u8){
        let to = 63 - to;
        let from = from as usize;
        let to = to as usize;
        let mut mask = CheckersBitboard::mask_position(from / 8, from % 8);
        let mut piece = 0;
        println!("MASK IS {:064b}", mask);
        print!("BLACK PIECES {:064b}", self.game.black_pieces);
        println!("MASK IS {}", self.game.black_pieces & mask);
        if self.game.black_pieces & mask != 0 {
            piece = 1;
        } else if self.game.white_pieces & mask != 0 {
            piece = 2;
        } else if self.game.black_kings & mask != 0 {
            piece = 3;
        } else if self.game.white_kings & mask != 0 {
            piece = 4;
        }
        match piece {
            1 => {
                self.game.black_pieces &= !mask;
                self.game.black_pieces |= 1u64 << to;
            }
            2 => {
                self.game.white_pieces &= !mask;
                self.game.white_pieces |= 1u64 << to;
            }
            3 => {
                self.game.black_kings &= !mask;
                self.game.black_kings |= 1u64 << to;
            }
            4 => {
                self.game.white_kings &= !mask;
                self.game.white_kings |= 1u64 << to;
            }
            _ => panic!("Invalid piece"),
        }
    }
    pub fn loadFromString(&mut self, boardString: &str) -> Result<CheckersBitboard, &'static str> {
        // split sting at whitespace
        let mut parts = boardString.split_whitespace();
        // check if there are 8 parts
        if parts.clone().count() != 2 {
            return Err("Invalid board state");
        }

        let state_string = parts.nth(0).unwrap();
        // let turn_string = parts.nth(1).unwrap();

        let mut game = CheckersBitboard::new();
        let mut row = 0;
        let mut col = 0;
        for c in state_string.chars() {
            match c {
                '/' => {
                    row += 1;
                    col = 0;
                }
                'b' | 'w' | 'B' | 'W' => {
                     match c {
                        'b' => self.game.set_position(row, col, Some(true), false),
                        'w' => self.game.set_position(row, col, Some(false), false),
                        'B' => self.game.set_position(row, col, Some(true), true),
                        'W' => self.game.set_position(row, col, Some(false), true),
                        _ => unreachable!(),
                    };
                    col += 1;
                }
                _ if c.is_digit(10) => {
                    let num = c.to_digit(10).unwrap() as i8;
                    for _ in 0..num {
                        self.game.set_position(row, col, None, false);
                        col += 1;
                    }
                }
                _ => return Err("Invalid Character found in board state character was "),
            }
        }
        Ok(game)
    }


}

pub struct Piece {
    pub is_black: bool,
    pub is_king: bool,
}


#[derive(Debug, Clone)]
pub struct CheckersBitboard {
    pub black_pieces: u64,
    pub white_pieces: u64,
    pub black_kings: u64,
    pub white_kings: u64,
}


impl CheckersBitboard {
    pub fn pregen_moves() -> [[u64; 64];3] {
        println!("Pregenerating moves...");
        let mut moves = [[0u64; 64]; 3];

        for i in 0..64 {
            let position:u64 = 1u64 << 63- i;
            //if we arent on the top row, or the left column or right
            if (i % 8 != 0) && (i > 7) && (i % 8 != 7) {
                moves[0][i] |= position << 7;
                moves[0][i] |= position << 9;
         
            }
            // if we are in left col but not top row
            else if (i % 8 == 0) && (i > 7) {
                moves[0][i] |= position << 9;
            }
            // if we are in right col but not top row
            else if (i % 8 == 7) && (i > 7) {
                moves[0][i] |= position << 7;
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



            moves[2][i] = moves[0][i] | moves[1][i];

        }
        
        moves
        
        }


    
    pub fn new() -> Self {
        Self {
            black_pieces: 0,
            white_pieces: 0,
            black_kings: 0,
            white_kings: 0,
    
        }
    }

    // pub fn get_moves(&self) -> Vec<CheckersBitboard>{
    //     //    Construct list of moves, where each move is a bitboard
    //     // containing the current location and the moved location.
    //     //    ex. 0x11 = 10001, a piece at 1 and a piece at 1 << 4.

    //     let empty_square = !(self.black_pieces | self.white_pieces | self.black_kings | self.white_kings);
    //     let mut moves = Vec::new();

        

    // } 
    pub fn get_jumps(&self, sideWhite:bool){
        let empty_squares = !(self.black_pieces | self.white_pieces | self.black_kings | self.white_kings);
        // let mut moves = Vec::new();
        let takeable = (empty_squares >>7 ) & if !sideWhite {self.black_pieces | self.black_kings} else {self.white_pieces | self.white_kings};
        Self::pretty_print_bitboard(takeable);
        // print!("{:064b}", self.white_pieces)
        println!();
        Self::pretty_print_bitboard(empty_squares);
        
        


    }
    pub fn pretty_print_bitboard(bitboard: u64) {
        let mut bitboard = bitboard;
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
    fn mask_position(row: usize, col: usize) -> u64 {
        1u64 << 63 - (row * 8 + col)
    }

    pub fn set_position(&mut self, row: usize, col: usize, black_piece: Option<bool>, king_piece: bool) {
        let mask = Self::mask_position(row, col);
        if black_piece.is_none() == false{
            if black_piece.unwrap() == false{
            println!("white piece at row: {}, col: {}", row, col);
            print!("mask is: ");
            print!("{:064b}", mask);

            }
        }

        //check if black_piece is None
        if( black_piece.is_none()){
            self.black_pieces &= !mask;
            self.white_pieces &= !mask;
            self.black_kings &= !mask;
            self.white_kings &= !mask;
            return;
        }

        let black_piece = black_piece.unwrap();
        if king_piece{
            if  black_piece{
                self.black_kings |= mask;
            }else{
                self.white_kings |= mask;
            }
        }
        else{
            if  black_piece{
                self.black_pieces |= mask;
            }else{
                self.white_pieces |= mask;
            }
        }



        
    }
        pub fn printBoard(&self) {
        let numsToLetters = [' ', '○', '●', '○', '●'];
        println!("|---|---|---|---|---|---|---|---|");
        for row in 0..8 {
            print!("|");
            for col in 0..8 {
                let mask = Self::mask_position(row, col);
                let mut piece = 0;
                if self.black_pieces & mask != 0 {
                    piece = 1;
                } else if self.white_pieces & mask != 0 {
                    piece = 2;
                } else if self.black_kings & mask != 0 {
                    piece = 3;
                } else if self.white_kings & mask != 0 {
                    piece = 4;
                }
                print!(" {} |", numsToLetters[piece]);
            }
            println!("\n|---|---|---|---|---|---|---|---|");
        }
    }

}    
// impl CheckersBitboard {


    




//     pub fn loadFromString(boardString: &String) -> Result<CheckersBitboard, &'static str> {
//         // split sting at whitespace
//         let mut parts = boardString.split_whitespace();
//         // check if there are 8 parts
//         if parts.clone().count() != 2 {
//             return Err("Invalid board state");
//         }

//         let state_string = parts.nth(0).unwrap();
//         // let turn_string = parts.nth(1).unwrap();

//         let mut game = CheckersBitboard::new();
//         let mut row = 0;
//         let mut col = 0;
//         for c in state_string.chars() {
//             match c {
//                 '/' => {
//                     row += 1;
//                     col = 0;
//                 }
//                 'b' | 'w' | 'B' | 'W' => {
//                     game.board[row][col] = match c {
//                         'b' => 2,
//                         'w' => 1,
//                         'B' => 4,
//                         'W' => 3,
//                         _ => unreachable!(),
//                     };
//                     col += 1;
//                 }
//                 _ if c.is_digit(10) => {
//                     let num = c.to_digit(10).unwrap() as i8;
//                     for _ in 0..num {
//                         game.board[row][col] = 0;
//                         col += 1;
//                     }
//                 }
//                 _ => return Err("Invalid Character found in board state character was "),
//             }
//         }
//         Ok(game)
//     }

//     pub fn printBoard(&self) {
//         let numsToLetters = [' ', '○', '●', '○', '●'];
//         println!("|---|---|---|---|---|---|---|---|");
//         for row in self.board.iter() {
//             print!("| ");
//             for col in row.iter() {
//                 print!("{} | ", numsToLetters[*col as usize]);
//             }
//             println!("\n|---|---|---|---|---|---|---|---|");
//         }
//     }
// }
