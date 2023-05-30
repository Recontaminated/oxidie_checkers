use crate::{
    generation,
    utils::{self},
};
// wrapper class that holds bitboard. mostly holds methods for interacting with bitboard
pub struct GameState {
    pub game: CheckersBitboard,
}
// public methods for GameState
impl GameState {
    // loads the starting position from a file or string
    pub fn starting_pos() -> GameState {
        // open file
        // let file = fs::read_to_string("src/startState.txt").expect("Unable to read file");
        const STARTING_STRING: &str = "1b1b1b1b/b1b1b1b1/1b1b1b1b/8/8/w1w1w1w1/1w1w1w1w/w1w1w1w1 w";
        let game = CheckersBitboard::new(true);
        let mut state = GameState { game: game };
        state.load_from_string(STARTING_STRING).unwrap();
        state
    }

    pub fn load_from_string(
        self: &mut GameState,
        board_string: &str,
    ) -> Result<CheckersBitboard, &'static str> {
        // split sting at whitespace
        let mut parts = board_string.split_whitespace();
        // check if there are 8 parts
        if parts.clone().count() != 2 {
            return Err("Invalid board state");
        }

        let state_string = parts.nth(0).unwrap();

        let game = CheckersBitboard::new(true);
        let mut row = 0;
        let mut col = 0;
        //rust's type system is really nice here because it forces us to handle all cases
        for c in state_string.chars() {
            // if we are at the end of the row, go to the next row
            match c {
                '/' => {
                    row += 1;
                    col = 0;
                }
                // if we are at a piece, set the piece
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
                // if we are at a number, skip that many squares
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
// move needs to be able to be equal to other move, and needs to be able to be printed. Derive is almost like java interface
//move holds list of submoves incase of forced capture
#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    sub_moves: Vec<(u8, u8)>, //from,tp
}
impl Move {
    fn new(from: u8, to: u8) -> Move {
        Move {
            sub_moves: vec![(from, to)],
        }
    }
    // uci move notation is a string of 4 characters, the first 2 are the from square, the last 2 are the to square
    pub fn from_uci(uci: Vec<&str>) -> Result<Move, &'static str> {
        // - normal move:      `move e3f4`
        // - single capture:   `move f2d4`
        // - multiple capture: `move f2d4f6`
        // - multiple moves:   `move d4e3 f2d4f6`
        let mut submoves: Vec<(u8, u8)> = Vec::new();
        for i in 0..uci.len() {
            let from = &uci[i][0..2];
            let to = &uci[i][2..4];
            let from = match utils::letter_coord_to_index(from) {
                Ok(x) => x,
                Err(e) => return Err(e),
            };
            let to = match utils::letter_coord_to_index(to) {
                Ok(x) => x,
                Err(e) => return Err(e),
            };
            submoves.push((from, to));
        }

        Ok(Move {
            sub_moves: submoves,
        })
    }
    // we need to use string slices because rust wont let us move out of self
    // what this does is it takes the moves from the other move and appends them to the end of this move
    fn append_moves_from(self: &mut Move, other: &Move) {
        self.sub_moves.extend_from_slice(&other.sub_moves);
    }
    // nice pretty print for move
    fn print(self: &Move) -> String {
        let conversion = [
            "A8", "B8", "C8", "D8", "E8", "F8", "G8", "H8", "A7", "B7", "C7", "D7", "E7", "F7",
            "G7", "H7", "A6", "B6", "C6", "D6", "E6", "F6", "G6", "H6", "A5", "B5", "C5", "D5",
            "E5", "F5", "G5", "H5", "A4", "B4", "C4", "D4", "E4", "F4", "G4", "H4", "A3", "B3",
            "C3", "D3", "E3", "F3", "G3", "H3", "A2", "B2", "C2", "D2", "E2", "F2", "G2", "H2",
            "A1", "B1", "C1", "D1", "E1", "F1", "G1", "H1",
        ];
        let mut pretty_move = String::new();
        for (from, to) in self.sub_moves.iter() {
            pretty_move.push_str(conversion[*from as usize]);
            pretty_move.push_str(" -> ");
            pretty_move.push_str(conversion[*to as usize]);
            pretty_move.push_str("\n");
        }
        pretty_move
    }
}


/*
    Our bitboard consists of u64s TODO: refactor to only 3 and have king handle 1 bb
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CheckersBitboard {
    pub black_pieces: u64,
    pub white_pieces: u64,
    pub black_kings: u64,
    pub white_kings: u64,
    pub white_to_move: bool,
}

impl CheckersBitboard {
    pub fn new(white_to_move: bool) -> Self {
        Self {
            black_pieces: 0,
            white_pieces: 0,
            black_kings: 0,
            white_kings: 0,
            white_to_move: white_to_move,
        }
    }


    /*
        returns a vector of all the captures that can be made from a given square
     */
    pub fn get_captures(&self, square: u8, white_to_move: bool) -> Vec<Move> {
        //if white to move, we are looking for white pieces to capture black pieces
        let (attacking_pieces, defending_pieces, lookup_index) = if white_to_move {
            (
                self.white_pieces | self.white_kings,
                self.black_pieces | self.black_kings,
                0,
            )
        } else {
            (
                self.black_pieces | self.black_kings,
                self.white_pieces | self.white_kings,
                1,
            )
        };
        // create a bb of all occupied squares because we cant move into them
        let all_occ = attacking_pieces | defending_pieces;

        let mut moves: Vec<Move> = Vec::new();
        // bitboard mask of the piece we are looking at and if it is a king or if there is even a piece there
        let bb = 1u64 << (63 - square) & (attacking_pieces); // | self.white_kings | self.black_kings
        if bb == 0 {
            return moves;
        }
        
  

        let att = if (1u64 << (63 - square)) & (self.white_kings | self.black_kings) != 0 {
            generation::LOOKUP_TABLE.all_capturing_moves[2][square as usize]
        } else {
            generation::LOOKUP_TABLE.all_capturing_moves[lookup_index][square as usize]
        };


        //we can only move into non occupied squares
        let mut att_temp = att & !all_occ;
        while att_temp != 0 {
            // grab a move we can make, and remove it from the bitboard of moves. this is the same as popping the lsb
            let end = utils::lsb_idx(&att_temp);
            att_temp &= att_temp - 1; //pop ths lsb

            // we can only make capture if there is a piece in between the start and end square
            if (1u64 << (63 - (((square) + end as u8) / 2)) & defending_pieces) != 0 {
                let partial_move = Move::new(square, end as u8);
                // make a copy of the board and apply the move to it
                let mut next_bitboard = *self;
                next_bitboard.move_piece(square, end as u8);
                //recursively call get captures on the new board
                let temp_moves = Self::get_captures(&next_bitboard, end as u8, white_to_move);
                // if there are no more captures, or we are at the end of the board, add the move to the list of moves
                if temp_moves.is_empty() || (end / 8 == 0 || end / 8 == 7) { //TODO: can we capture once we move into king and then go back?????
                    moves.push(partial_move);
                } 
                else {
                    //chained capture, we need to append the moves from the recursive call to the current move
                    for temp_move in temp_moves {
                        let mut concat_move = partial_move.clone();
                        concat_move.append_moves_from(&temp_move);
                        moves.push(concat_move);
                    }
                }
            }
        }

        moves
    }
    // uses get captures to get all captures from all pieces
    /*
        returns a vector of all the captures that can be made from game state
     */
    pub fn get_all_captures(self: &CheckersBitboard, white_to_move: bool) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        let pieces_to_move = if white_to_move {
            self.white_pieces | self.white_kings
        } else {
            self.black_pieces | self.black_kings
        };

        let mut pieces_to_move_copy = pieces_to_move;
       

       // go over all pieces that can move, see if they have captures
        while pieces_to_move_copy != 0 {
            let lsb_index = 63 - pieces_to_move_copy.trailing_zeros();

            pieces_to_move_copy &= pieces_to_move_copy - 1; // pop lsb
            if white_to_move != self.white_to_move{
                panic!("white to move is not equal to self.white_to_move")

            }
            let temp_moves = self.get_captures(lsb_index as u8, white_to_move);

            for temp_move in temp_moves {
                moves.push(temp_move);
            }
        }

        moves
    }

    /*
        returns a vector of all the non captures that can be made from game state
     */
    pub fn get_non_capture_moves(self: &CheckersBitboard, white_to_move: bool) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        let all_pieces =
            self.white_pieces | self.white_kings | self.black_pieces | self.black_kings;

        let pieces_to_move = if white_to_move {
            self.white_pieces | self.white_kings
        } else {
            self.black_pieces | self.black_kings
        };

        let mut pieces_to_move_copy = pieces_to_move;

        while pieces_to_move_copy != 0 {
            let lsb_index = 63 - pieces_to_move_copy.trailing_zeros();
            let is_king = (1u64 << (63 - lsb_index)) & (self.white_kings | self.black_kings) != 0;
            pieces_to_move_copy &= pieces_to_move_copy - 1; // pop lsb
                                                            // Self::pretty_print_bitboard(pieces_to_move_copy);
                                                            // println!();
            let pushes;

            // query lookup table depending on if we are a king or not amd what color it is
            if is_king {
                pushes = generation::LOOKUP_TABLE.all_non_capturing_moves[2][lsb_index as usize];
            } else {
                pushes = if white_to_move {
                    generation::LOOKUP_TABLE.all_non_capturing_moves[0][lsb_index as usize]
                } else {
                    generation::LOOKUP_TABLE.all_non_capturing_moves[1][lsb_index as usize]
                }
            }
            let mut att_temp = pushes & !all_pieces; // bitboard of all moves not occupied by any piece
            while att_temp != 0 {
                let end = utils::lsb_idx(&att_temp);
                att_temp &= att_temp - 1;

                moves.push(Move::new(lsb_index as u8, end as u8));
            }
        }
        moves
    }

    /* 
    returns combined vec of all captures and non captures. If we have captures, we can only make captures
    */
    pub fn get_all_legal_moves(self: &CheckersBitboard) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        let white_to_move = self.white_to_move;

        let captures = self.get_all_captures(white_to_move);
        if captures.is_empty() {
            let non_captures = self.get_non_capture_moves(white_to_move);
            for non_capture in non_captures {
                moves.push(non_capture);
            }
        } else {
            for capture in captures {
                moves.push(capture);
            }
        }

        moves
    }
    // helper function to create a bitboard mask of a position
    fn mask_position(row: usize, col: usize) -> u64 {
        1u64 << 63 - (row * 8 + col)
    }
     
     fn set_position(
        &mut self,
        row: usize,
        col: usize,
        black_piece: Option<bool>,
        king_piece: bool,
    ) {
        let mask = Self::mask_position(row, col);

        // remove the piece from all bitboards so we dont have to look for it
        if black_piece.is_none() {
            self.black_pieces &= !mask;
            self.white_pieces &= !mask;
            self.black_kings &= !mask;
            self.white_kings &= !mask;
            return;
        }

        let black_piece = black_piece.unwrap();// currently this will panic if we try to set a piece to none probably behavior we dont want?
        if king_piece {
            if black_piece {
                self.black_kings |= mask;
            } else {
                self.white_kings |= mask;
            }
        } else {
            if black_piece {
                self.black_pieces |= mask;
            } else {
                self.white_pieces |= mask;
            }
        }
    }
    // goes through all submoves of a move and applies them to the board (eg. captures)
    pub fn apply_move(self: &mut CheckersBitboard, mov: &Move) {
        for sub_move in &mov.sub_moves {
            let (from, to) = sub_move;
            self.move_piece(*from, *to);
        }
        // self.white_to_move = !self.white_to_move;
    }
//   human readable way of moving a piece on board handles promotion and validity
    pub fn move_piece(self: &mut CheckersBitboard, from: u8, to: u8) {
        let from = from as usize;
        let to = to as usize;
        let mask = 1u64 << (63 - from);

        let mut piece_type = -1; // 0 = white piece, 1= white king, 2 = black piece, 3 = black king
        let mut promote = false;
            // figure out what type of piece we are moving
        if self.white_to_move {
            if mask & self.white_pieces != 0 {
                piece_type = 0;
            } else if mask & self.white_kings != 0 {
                piece_type = 1;
            }

            if piece_type == 0 && to < 8 {
                promote = true;
            }
        } else {
            if mask & self.black_pieces != 0 {
                piece_type = 2;
            } else if mask & self.black_kings != 0 {
                piece_type = 3;
            }

            if piece_type == 2 && to > 55 {
                promote = true;
            }
        }
        // make the moves
        match piece_type {
            2 => {
                self.black_pieces &= !mask;
                if !promote {
                    self.black_pieces |= 1u64 << (63 - to);
                } else {
                    self.black_kings |= 1u64 << (63 - to);
                }
            }
            0 => {
                self.white_pieces &= !mask;
                if !promote {
                    self.white_pieces |= 1u64 << (63 - to);
                } else {
                    self.white_kings |= 1u64 << (63 - to);
                }
            }
            3 => {
                self.black_kings &= !mask;
                self.black_kings |= 1u64 << (63 - to);
            }
            1 => {
                self.white_kings &= !mask;
                self.white_kings |= 1u64 << (63 - to);
            }
            _ => panic!("Invalid piece"),
        }
        // we have to do this last or https://images.duckarmada.com/ckKsmYmwsnDw
        // just checking if we captured a piece by seeing if the distance between the from and to square is greater than 9
        if from.abs_diff(to) > 9 {
            let mask = 1u64 << (63 - (from + to) / 2);
            //apply mask to all bitboards to remove the piece we captured
            self.black_pieces &= !mask;
            self.white_pieces &= !mask;
            self.black_kings &= !mask;
            self.white_kings &= !mask;
        }
    }
    // helper function to print bitboard in a human readable way
    pub fn print_board(&self, print_nums: bool) {
        let nums_to_letters = [' ', '○', '●', 'B', 'W'];
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
                if piece == 0 {
                    if print_nums {
                        //print the square number but dont make the numbers overflow
                        if row * 8 + col < 10 {
                            print!(" {} |", row * 8 + col);
                        } else {
                            print!("{} |", row * 8 + col);
                        }
                        continue;
                    } else {
                        print!("   |");
                        continue;
                    }
                }
                print!(" {} |", nums_to_letters[piece]);
            }
            println!("\n|---|---|---|---|---|---|---|---|");
        }
    }
}

