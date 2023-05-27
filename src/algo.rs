use crate::{board::{CheckersBitboard, Move}, utils::{pretty_print_bitboard, self}};

pub fn evaluation(bitboard: &CheckersBitboard)-> i32{
    // returns from perspective of white
    let mut value = 0;
    let mut white_pieces = bitboard.white_pieces;
    let mut black_pieces = bitboard.black_pieces;
    let mut white_kings = bitboard.white_kings;
    let mut black_kings = bitboard.black_kings;

    let white_piece_valuation = [
    7,7,7,7,7,7,7,7,
    6,6,6,6,6,6,6,6,
    5,5,5,5,5,5,5,5,
    4,4,4,4,4,4,4,4,
    3,3,3,3,3,3,3,3,
    2,2,2,2,2,2,2,2,
    1,1,1,1,1,1,1,1,
    0,0,0,0,0,0,0,0
    ];
    let black_piece_valuation = [
        0,0,0,0,0,0,0,0,
        1,1,1,1,1,1,1,1,
        2,2,2,2,2,2,2,2,
        3,3,3,3,3,3,3,3,
        4,4,4,4,4,4,4,4,
        5,5,5,5,5,5,5,5,
        6,6,6,6,6,6,6,6,
        7,7,7,7,7,7,7,7,
    ];
    // utils::pretty_print_bitboard(black_pieces);
    // println!();
    // utils::pretty_print_bitboard(white_pieces);
    while white_pieces != 0 {
        value += 100;
        value += white_piece_valuation[utils::lsb_idx(&white_pieces) as usize];
        white_pieces &= white_pieces - 1;
    }
    while black_pieces != 0 {
        value -= 100;
        value -= black_piece_valuation[utils::lsb_idx(&black_pieces) as usize];

        black_pieces &= black_pieces - 1;
    }
    while white_kings != 0 {
        value += 600;
        white_kings &= white_kings - 1;
    }
    while black_kings != 0 {
        value -= 600;
        black_kings &= black_kings - 1;
    }



    println!("eval returning {}", value);
    // panic!("fucky wucky");
    value

    
}
pub fn negamax(bitboard: &CheckersBitboard, depth: i32, alpha: i32, beta: i32, iswhite:bool, nodes_counter:&mut i32) -> i32 {

    //https://www.chessprogramming.org/Negamax


    *nodes_counter += 1;
    if depth == 0 {
        
        if bitboard.get_all_captures(iswhite).len() == 0 {
            return if iswhite{ 1} else {-1} * evaluation(bitboard);
        }
        else {

            return -quiescence(bitboard, depth, alpha, beta, !iswhite, nodes_counter)
        }
    }


    let mut child_nodes = bitboard.get_all_legal_moves();// TODO: could speed up here by sharing
    let mut value = -99999;
    let mut alpha = alpha;

    for child_move in child_nodes {
        let mut next_bitboard = bitboard.clone();
        next_bitboard.printBoard();
        println!("running move {:?}", child_move);
        next_bitboard.apply_move(&child_move);
        next_bitboard.printBoard();
        
        // println!("{}",negamax(&next_bitboard, depth - 1, -beta, -alpha, !iswhite, nodes_counter));
        let child_value = -negamax(&next_bitboard, depth - 1, -beta, -alpha, !iswhite, nodes_counter);
        println!("nega returned {:?}", child_value);
        value = value.max(child_value);
        alpha = alpha.max(value);
  
        if alpha >= beta {
            // print!("pruned");
            break;
        }
        bitboard.printBoard();
        panic!("die")
    }

    value
}
pub fn quiescence(bitboard: &CheckersBitboard, depth: i32,mut alpha: i32, mut beta: i32, iswhite:bool, nodes_counter:&mut i32) -> i32 {
    
    *nodes_counter += 1;

//https://en.wikipedia.org/wiki/Quiescence_search
    let captures = bitboard.get_all_captures(iswhite);
    if depth == 0 || captures.len() == 0 {
        return if iswhite {1} else {-1} * evaluation(bitboard);
    }
  //check for game over by looking at all moves
  let mut value = -999999;
    for child_move in captures{ 
        let mut next_bitboard = *bitboard;
        next_bitboard.apply_move(&child_move);
        let child_value = -quiescence(&next_bitboard, depth - 1, -beta, -alpha, !iswhite, nodes_counter);
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
pub fn get_best_move(bitboard: &CheckersBitboard,depth: i32){
    let moves:Vec<Move> = bitboard.get_all_legal_moves();
    let mut evaled_moved:Vec<EvaluatedMove> = Vec::new();
    let mut nodes = 0;
    for child_move in moves {
        let mut next_bitboard = *bitboard;
        next_bitboard.apply_move(&child_move);
        let child_value = -negamax(&next_bitboard, depth - 1, -99999, 99999,!bitboard.white_to_move, &mut nodes);
        
        evaled_moved.push(EvaluatedMove{mov:child_move, value:child_value});

    }

    evaled_moved.sort_by(|a, b| b.value.cmp(&a.value));
    println!("move evaled stuff");
    evaled_moved.iter().for_each(|f| {
        println!("{:?}", f)
});
    println!("Nodes: {}", nodes);
    println!("Best move: {:?}", evaled_moved[0].mov);
    println!("Value: {}", evaled_moved[0].value);


}