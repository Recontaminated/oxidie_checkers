use crate::{board::{CheckersBitboard, Move}, utils::{pretty_print_bitboard, self}};

pub fn evaluation(bitboard: &CheckersBitboard)-> i32{
    // returns from perspective of white
    let mut value = 0;
    let mut pieces = [bitboard.white_pieces, bitboard.white_kings, bitboard.black_pieces, bitboard.black_kings];
 let man_values = 
[
     0,  0,  0,  0,  0,  0,  0,  0,
    24,  0, 10,  0,  8,  0,  9,  0,
     0,  8,  0, 10,  0, 10,  0, 24,
     3,  0,  4,  0,  5,  0,  1,  0,
     0,  2,  0,  13,  0,  8,  0,  1,
     1,  0,  2,  0,  1,  0,  0,  0,
     0, -1,  0,  7,  0,  9,  0, 25,
    -5,  0, 23,  0, 10,  0, 32,  0
];

let king_values =
[
     0, -7,  0, -1,  0, -3,  0,-10,
    -5,  0,  0,  0, -1,  0, -1,  0,
     0,  1,  0, 10,  0,  8,  0, -1,
    -1,  0, 10,  0, 11,  0,  1,  0,
     0,  2,  0, 11,  0, 10,  0, -1,
    -1,  0,  8,  0,  10, 0,  0,  0,
     0, -1,  0, -1,  0, -1,  0, -1,
   -10,  0,  0,  0, -1,  0, -7,  0
];

let piece_value = [100, 1000, -100, -1000];

    for x in 0..4{

        while pieces[x] != 0{

            let square = pieces[x].trailing_zeros() as usize;
            pieces[x] &= !(1 << square); // remove the piece from the bitboard
            value += piece_value[x];

            match x {  
                0 => value += man_values[square],
                1 => value += king_values[square],
                2 => value -= man_values[63-square],
                3 => value -= king_values[63-square],
                _ => panic!("Invalid piece type"),
                
            }

        }

    }
   
    value

    
}
pub fn negamax(bitboard: &CheckersBitboard, depth: i32, alpha: i32, beta: i32, iswhite:bool, nodes_counter:&mut i32) -> i32 {

    //https://www.chessprogramming.org/Negamax
    *nodes_counter += 1;
    if *nodes_counter % 100000  ==0{
        println!("Moves {}", *nodes_counter);
    }

    if depth == 0 {
        
        if bitboard.get_all_captures(iswhite).len() == 0 {
            return if iswhite{ 1} else {-1} * evaluation(bitboard);
        }
        else {
       
            return -quiescence(bitboard, depth, -alpha, -beta, !iswhite, nodes_counter)
        }
    }


    let mut child_nodes = bitboard.get_all_legal_moves();// TODO: could speed up here by sharing
    let mut value = -99999;
    let mut alpha = alpha;

    for child_move in child_nodes {

        let mut next_bitboard = *bitboard;
        // println!("next bitboard");
        // // next_bitboard.printBoard();
        next_bitboard.apply_move(&child_move);

        
        // println!("{}",negamax(&next_bitboard, depth - 1, -beta, -alpha, !iswhite, nodes_counter));
        let child_value = -negamax(&next_bitboard, depth - 1, -beta, -alpha, next_bitboard.white_to_move, nodes_counter);
        value = value.max(child_value);
        alpha = alpha.max(value);
  
        if alpha >= beta {
            // print!("pruned");
            break;
        }

    }

    value
}
pub fn quiescence(bitboard: &CheckersBitboard, depth: i32,mut alpha: i32, mut beta: i32, iswhite:bool, nodes_counter:&mut i32) -> i32 {
    
    *nodes_counter += 1;

//https://en.wikipedia.org/wiki/Quiescence_search
    let captures = bitboard.get_all_captures(!iswhite);//TODO: WHAT
    if depth == 0 || captures.len() == 0 {
        return if iswhite {1} else {-1} * evaluation(bitboard);
    }
  //check for game over by looking at all moves
  let mut value = -999999;
    for child_move in captures{ 
        let mut next_bitboard = *bitboard;
        next_bitboard.apply_move(&child_move);
        let child_value = -quiescence(&next_bitboard, depth - 1, -beta, -alpha, next_bitboard.white_to_move, nodes_counter);
        value = value.max(child_value);
        alpha = alpha.max(value);
        if alpha >= beta {
            break;
        }
    }
    value

}
#[derive(Debug)]
struct EvaluatedMove {
    mov: Move,
    value: i32,
}
pub fn get_best_move(bitboard: &CheckersBitboard,depth: i32) -> Move{
    let moves:Vec<Move> = bitboard.get_all_legal_moves();
    let mut evaled_moved:Vec<EvaluatedMove> = Vec::new();
    let mut nodes = 0;
    for child_move in moves {
        let mut next_bitboard = *bitboard;
        next_bitboard.apply_move(&child_move);
        let child_value = -negamax(&next_bitboard, depth - 1, -99999, 99999,next_bitboard.white_to_move, &mut nodes);
          
        evaled_moved.push(EvaluatedMove{mov:child_move, value:child_value});

    }

    evaled_moved.sort_by(|a, b| b.value.cmp(&a.value));
//     println!("move evaled stuff");
//     evaled_moved.iter().for_each(|f| {
//         println!("{:?}", f)
// });
    println!("Nodes: {}", nodes);
    println!("Best move: {:?}", evaled_moved[0].mov);
    println!("Value: {}", evaled_moved[0].value);
    evaled_moved[0].mov.clone()

}