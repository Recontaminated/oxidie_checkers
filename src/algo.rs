use crate::board::{CheckersBitboard, Move};

pub fn evaluation(bitboard: &CheckersBitboard)-> i32{
    // returns from perspective of white
    let mut value = 0;
    let mut white_pieces = bitboard.white_pieces;
    let mut black_pieces = bitboard.black_pieces;
    let mut white_kings = bitboard.white_kings;
    let mut black_kings = bitboard.black_kings;

    while white_pieces != 0 {
        value += 1;
        white_pieces &= white_pieces - 1;
    }
    while black_pieces != 0 {
        value -= 1;
        black_pieces &= black_pieces - 1;
    }
    while white_kings != 0 {
        value += 2;
        white_kings &= white_kings - 1;
    }
    while black_kings != 0 {
        value -= 2;
        black_kings &= black_kings - 1;
    }
    value

    
}
pub fn negamax(bitboard: &CheckersBitboard, depth: i32, alpha: i32, beta: i32, iswhite:bool, nodes_counter:&mut i32) -> i32 {
    *nodes_counter += 1;
    if depth == 0 {
        return if iswhite{ 1} else {-1} * evaluation(bitboard);
    }


    let mut child_nodes = bitboard.get_all_legal_moves();
    let mut value = -99999;
    let mut alpha = alpha;

    for child_move in child_nodes {
        let mut next_bitboard = *bitboard;
        next_bitboard.apply_move(&child_move);
        // println!("{}",negamax(&next_bitboard, depth - 1, -beta, -alpha, !iswhite, nodes_counter));
        let child_value = -negamax(&next_bitboard, depth - 1, -beta, -alpha, !iswhite, nodes_counter);
        value = value.max(child_value);
        alpha = alpha.max(value);

        if alpha >= beta {
            // print!("pruned");
            break;
        }
    }

    value
}

struct EvaluatedMove {
    mov: Move,
    value: i32,
}
pub fn get_best_move(bitboard: &CheckersBitboard,depth: i32){
    let moves:Vec<Move> = bitboard.get_all_legal_moves();
    let mut evaledMoved:Vec<EvaluatedMove> = Vec::new();
    let mut nodes = 0;
    for child_move in moves {
        let mut next_bitboard = *bitboard;
        next_bitboard.apply_move(&child_move);
        let child_value = -negamax(&next_bitboard, depth - 1, -99999, 99999, false, &mut nodes);
        
        evaledMoved.push(EvaluatedMove{mov:child_move, value:child_value});

    }

    evaledMoved.sort_by(|a, b| b.value.cmp(&a.value));
    println!("Nodes: {}", nodes);
    println!("Best move: {:?}", evaledMoved[0].mov);
    println!("Value: {}", evaledMoved[0].value);


}