/*
    This is the module that handles checker engine logic
    includes evaluation, negamax, and quiescence
 */
use crate::{board::{CheckersBitboard, Move}, transposition::TranspositionTable};

pub fn evaluation(bitboard: &CheckersBitboard) -> i32 {
    // returns from perspective of white
    let mut value = 0;
    let mut pieces = [
        bitboard.white_pieces,
        bitboard.white_kings,
        bitboard.black_pieces,
        bitboard.black_kings,
    ];

    // we want backrank to be filled with pieces, but also value center of board and pushing
    let man_values = [
        0, 0, 0, 0, 0, 0, 0, 0,
        24,0, 15, 0, 10, 0, 15, 0,
        0, 8, 0, 10, 0, 10, 0, 25,
        3, 0, 3, 0, 5, 0, 1, 0,
        0, 2, 0, 15, 0, 10, 0, 0,
        1, 0, 2, 0, 1, 0, 0, 0, 
        0, 0, 0, 7, 0, 9, 0, 25,
        0, 0, 25, 0, 10, 0, 30, 0,
    ];
// in general we want to control the center of the board, otherwise kings are not helpful near edges 
    let king_values = [
        0, -10, 0, -5, 0, -5, 0, -10,
        -5, 0, 0, 0, -1, 0, -1, 0,
        0, 1, 0, 10, 0, 8, 0, -10,
        -5, 0, 10, 0, 11, 0, 1, 0,
        0, 2, 0, 11, 0, 10, 0, -10,
        -5, 0, 8, 0, 10, 0, 0, 0,
         0,-1, 0, -1, 0, -1, 0, -10,
        -10,0, 0, 0, -1, 0, -7, 0,
    ];
    // sacraficing pieces is very painful
                        //white king black king
    let piece_value = [100, 1000, -100, -1000];
// go through each position on table and add up the value of the pieces
    for x in 0..4 {
        while pieces[x] != 0 {
            let square = pieces[x].trailing_zeros() as usize;
            pieces[x] &= !(1 << square); // remove the piece from the bitboard
            value += piece_value[x];

            match x {
                0 => value += man_values[square],
                1 => value += king_values[square],
                2 => value -= man_values[63 - square],
                3 => value -= king_values[63 - square],
                _ => panic!("Invalid piece type"),
            }
        }
    }

    value
}
/*
recursive simpiflied minimax search algorithm
 */
pub fn negamax(
    bitboard: &CheckersBitboard,
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
    iswhite: bool,
    nodes_counter: &mut i32,transposition_table:&mut TranspositionTable
) -> i32 {
    //https://www.chessprogramming.org/Negamax


    // keep track of how many nodes we have visited
    *nodes_counter += 1;


    let orginal_alpha = alpha;
    

    // if we are at the bottom of the tree, return the evaluation of the board if the board has no more captures otherwise return the quiescence search
    if depth == 0 {
     
        if bitboard.get_all_captures(iswhite).len() == 0{
            return if iswhite { 1 } else { -1 } * evaluation(bitboard);
        } else {
            return -quiescence(bitboard, 2, -alpha, -beta, iswhite, nodes_counter);

        }
    }
// see if we have a cached value for this position
// https://stackoverflow.com/questions/69023024/understanding-negamax-with-transposition-tables
    match transposition_table.get(bitboard) {
        Some(cached_position) => {
            if cached_position.depth as i32 >= depth {
                
                if cached_position.flag == crate::transposition::flag::EXACT {
            
                    return cached_position.value;
                }
                else

                if cached_position.flag == crate::transposition::flag::LOWERBOUND && cached_position.value > alpha {
                    alpha = cached_position.value;
                }
                
                else if cached_position.flag == crate::transposition::flag::UPPERBOUND && cached_position.value < beta {
                    beta = cached_position.value;
                }

            


                if alpha >= beta {
                    return cached_position.value;
                }
            }
        }
        None => {}
    }

    let child_nodes = bitboard.get_all_legal_moves(); // TODO: could speed up here by sharing
    // large negative number to start
    let mut best_value = -9999999;
    let mut alpha = alpha;

    for child_move in child_nodes {

        // by dereferencing the bitboard we can avoid expensive cloning. this allows us to unmake move
        let mut next_bitboard = *bitboard;
        next_bitboard.apply_move(&child_move);
        // switch turns
        next_bitboard.white_to_move = !next_bitboard.white_to_move;
        
        let child_value = -negamax(
            &next_bitboard,
            depth - 1,
            -beta,
            -alpha,
            next_bitboard.white_to_move,
            nodes_counter,
            transposition_table
        );
        best_value = best_value.max(child_value);
        alpha = alpha.max(best_value);

        if alpha >= beta {
            // print!("pruned");
  
            break;
        }
    }

    //store the value in the transposition table
    if best_value <= orginal_alpha{
        transposition_table.put(bitboard, crate::transposition::CachedValue{
            value: best_value,
            depth: depth as u8,
            flag: crate::transposition::flag::UPPERBOUND,
        })
    }
    else if best_value >= beta{
        transposition_table.put(bitboard, crate::transposition::CachedValue{
            value: best_value,
            depth: depth as u8,
            flag: crate::transposition::flag::LOWERBOUND,
        })
    }
    else{
        transposition_table.put(bitboard, crate::transposition::CachedValue{
            value: best_value,
            depth: depth as u8,
            flag: crate::transposition::flag::EXACT,
        })
    }
        
    
    best_value
}

/*
    helps to avoid the horizon effect, where the engine will make a move that looks good but is actually bad, or vice versa 
 */
pub fn quiescence(
    bitboard: &CheckersBitboard,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    iswhite: bool,
    nodes_counter: &mut i32,
) -> i32 {
    // *nodes_counter += 1;

    //https://en.wikipedia.org/wiki/Quiescence_search
    let captures = bitboard.get_all_captures(iswhite); 
    if depth == 0 || captures.len() == 0 {
        return if iswhite { 1 } else { -1 } * evaluation(bitboard);
    }


    //essentially just minimax but only looking at captures
    let mut value = -999999;
    for child_move in captures {
        let mut next_bitboard = *bitboard;
        next_bitboard.apply_move(&child_move);
        next_bitboard.white_to_move = !next_bitboard.white_to_move;

        let child_value = -quiescence(
            &next_bitboard,
            depth - 1,
            -beta,
            -alpha,
            next_bitboard.white_to_move,
            nodes_counter,
        );
        value = value.max(child_value);
        alpha = alpha.max(value);
        if alpha >= beta {
            break;
        }
    }

    value
}

//debug trait allows us to print out the moves without defining a custom print function
#[derive(Debug)]
struct EvaluatedMove {
    mov: Move,
    value: i32,
}
/*
    returns the best move for the current position
 */
pub fn get_best_move(bitboard: &CheckersBitboard, depth: i32, transposition_table:&mut TranspositionTable) -> Move {
    let moves: Vec<Move> = bitboard.get_all_legal_moves();
    let mut evaled_moved: Vec<EvaluatedMove> = Vec::new();
    let mut nodes = 0;
    for child_move in moves {
        let mut next_bitboard = *bitboard;
        next_bitboard.apply_move(&child_move);
        next_bitboard.white_to_move = !next_bitboard.white_to_move;
        
        let child_value = -negamax(
            &next_bitboard,
            depth - 1,
            -99999,
            99999,
            next_bitboard.white_to_move,
            &mut nodes,
            transposition_table
        );

        evaled_moved.push(EvaluatedMove {
            mov: child_move,
            value: child_value,
        });
    }

    evaled_moved.sort_by(|a, b| b.value.cmp(&a.value));
    //     println!("move evaled stuff");
    //     evaled_moved.iter().for_each(|f| {
    //         println!("{:?}", f)
    // });
    println!("Nodes: {}", nodes);
    // println!("Best move: {:?}", evaled_moved[0].mov);
    println!("Value: {}", evaled_moved[0].value);
    evaled_moved[0].mov.clone()
}
