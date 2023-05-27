use crate::{
    generation,
    utils::{self, pretty_print_bitboard},
};

pub struct GameState {
    pub game: CheckersBitboard,
}
impl GameState {
    pub fn startingPos() -> GameState {
        // open file
        // let file = fs::read_to_string("src/startState.txt").expect("Unable to read file");
        const startingString: &str = "1b1b1b1b/b1b1b1b1/1b1b1b1b/8/8/w1w1w1w1/1w1w1w1w/w1w1w1w1 w";
        let game = CheckersBitboard::new(true);
        let mut state = GameState { game: game };
        state.loadFromString(startingString);
        state
    }

    pub fn loadFromString(
        self: &mut GameState,
        boardString: &str,
    ) -> Result<CheckersBitboard, &'static str> {
        // split sting at whitespace
        let mut parts = boardString.split_whitespace();
        // check if there are 8 parts
        if parts.clone().count() != 2 {
            return Err("Invalid board state");
        }

        let state_string = parts.nth(0).unwrap();
        // let turn_string = parts.nth(1).unwrap();

        let game = CheckersBitboard::new(true);
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
    fn append_moves_from(self: &mut Move, other: &Move) {
        self.sub_moves.extend_from_slice(&other.sub_moves);
    }
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

pub struct Piece {
    pub is_black: bool,
    pub is_king: bool,
}

#[derive(Debug, Clone, Copy)]
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

    // pub fn get_moves(&self) -> Vec<CheckersBitboard>{
    //     //    Construct list of moves, where each move is a bitboard
    //     // containing the current location and the moved location.
    //     //    ex. 0x11 = 10001, a piece at 1 and a piece at 1 << 4.

    //     let empty_square = !(self.black_pieces | self.white_pieces | self.black_kings | self.white_kings);
    //     let mut moves = Vec::new();

    // }

    pub fn get_captures(&self, square: u8, white_to_move: bool) -> Vec<Move> {
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

        let all_occ = attacking_pieces | defending_pieces;

        let mut moves: Vec<Move> = Vec::new();

        let bb = 1u64 << (63 - square) & (attacking_pieces | self.white_kings | self.black_kings);
        if bb == 0 {
            return moves;
        }

        let att_men = generation::LOOKUP_TABLE.all_capturing_moves[lookup_index][square as usize];
        // utils::pretty_print_bitboard(att_men);
        let att_king = generation::LOOKUP_TABLE.all_capturing_moves[2][square as usize];

        let att = if (1u64 << (63 - square)) & (self.white_kings | self.black_kings) != 0 {
            att_king
        } else {
            att_men
        };
        // utils::pretty_print_bitboard(att);
        // println!("lookup index: {}", lookup_index);

        let mut att_temp = att & !all_occ;
        while att_temp != 0 {
            let end = utils::lsb_idx(&att_temp);
            att_temp &= att_temp - 1;

            if (1u64 << (63 - (((square) + end as u8) / 2)) & defending_pieces) != 0 {
                let partial_move = Move::new(square, end as u8);

                let mut next_bitboard = *self;
                next_bitboard.move_piece(square, end as u8);

                let temp_moves = Self::get_captures(&next_bitboard, end as u8, white_to_move);

                if temp_moves.is_empty() || (end / 8 == 0 || end / 8 == 7) {
                    moves.push(partial_move);
                } else {
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
    pub fn get_all_captures(self: &CheckersBitboard, white_to_move: bool) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        let pieces_to_move = if white_to_move {
            self.white_pieces | self.white_kings
        } else {
            self.black_pieces | self.black_kings
        };

        let mut pieces_to_move_copy = pieces_to_move;

        while pieces_to_move_copy != 0 {
            let lsb_index = 63 - pieces_to_move_copy.trailing_zeros();

            pieces_to_move_copy &= pieces_to_move_copy - 1; // pop lsb

            let temp_moves = self.get_captures(lsb_index as u8, white_to_move);

            for temp_move in temp_moves {
                moves.push(temp_move);
            }
        }

        moves
    }

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
            let isKing = (1u64 << (63 - lsb_index)) & (self.white_kings | self.black_kings) != 0;
            pieces_to_move_copy &= pieces_to_move_copy - 1; // pop lsb
                                                            // Self::pretty_print_bitboard(pieces_to_move_copy);
                                                            // println!();
            let pushes;
            if isKing {
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
    fn mask_position(row: usize, col: usize) -> u64 {
        1u64 << 63 - (row * 8 + col)
    }

    pub fn set_position(
        &mut self,
        row: usize,
        col: usize,
        black_piece: Option<bool>,
        king_piece: bool,
    ) {
        let mask = Self::mask_position(row, col);

        //check if black_piece is None
        if black_piece.is_none() {
            self.black_pieces &= !mask;
            self.white_pieces &= !mask;
            self.black_kings &= !mask;
            self.white_kings &= !mask;
            return;
        }

        let black_piece = black_piece.unwrap();
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
    pub fn apply_move(self: &mut CheckersBitboard, mov: &Move) {
        for sub_move in &mov.sub_moves {
            let (from, to) = sub_move;
            self.move_piece(*from, *to);
        }
        self.white_to_move = !self.white_to_move;
    }

    pub fn move_piece(self: &mut CheckersBitboard, from: u8, to: u8) {
        let from = from as usize;
        let to = to as usize;
        let mask = 1u64 << (63 - from);

        let mut piece_type = -1; // 0 = white piece, 1= white king, 2 = black piece, 3 = black king
        let mut promote = false;

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
        // println!("piece type: {} tried to move form {} to {}" , piece_type, from, to);
        // println!("it was {}", self.white_to_move);
        // println!("board state");
        // self.printBoard();
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
        if from.abs_diff(to) > 9 {
            let mask = 1u64 << (63 - (from + to) / 2);
            //apply mask to all bitboards
            self.black_pieces &= !mask;
            self.white_pieces &= !mask;
            self.black_kings &= !mask;
            self.white_kings &= !mask;
        }
    }

    pub fn printBoard(&self, print_nums: bool) {
        let numsToLetters = [' ', '○', '●', 'B', 'W'];
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
