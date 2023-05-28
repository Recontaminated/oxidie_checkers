use crate::{board::Move, algo::get_best_move};

mod utils;
mod board;
mod generation;
mod algo;
mod transposition;

/*
    Custom memory allocator, not required but recommended.
    During testing this resulted in a ~20% speed up in move generation.
    If you are having trouble compiling the engine for your target system
    you can try removing the two lines below.
    https://github.com/microsoft/mimalloc
*/
// #[global_allocator]
// static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    println!("Hello, world!");
    let mut board = board::GameState::starting_pos();
    board.game.print_board(false);
    // utils::pretty_print_bitboard(generation::LOOKUP_TABLE.all_capturing_moves[0][42]);
    loop {
        println!("side to move: {:?}", board.game.white_to_move);
    
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: Vec<&str> = input.split_whitespace().collect();
        if input.len() ==0{
            println!("invalid input");
            continue;
        }
        let command = input[0];
        let args = &input[1..];
        match command{
            "isready" => {
                print!("readyok");
            },
            "endgame" => {
                break;
            },
//             asks the engine to move (UCI-like coordinate notation):
// - normal move:      `move e3f4`
// - single capture:   `move f2d4`
// - multiple capture: `move f2d4f6`
// - multiple moves:   `move d4e3 f2d4f6`
            "move" => {
                let mut moves:Vec<Move> = Vec::new();
         
                let mov = match Move::from_uci(Vec::from(args)) {
                    Ok(mov) => mov,
                    Err(e) => {
                        println!("invalid move: {}", e);
                        continue;
                    }
                };
                
                
                println!("move: {:?}", mov);
                let legal_moves = board.game.get_all_legal_moves();

                if !legal_moves.contains(&mov){
                    println!("invalid move");
                    continue;
                }
        
               
                board.game.apply_move(&mov);
                board.game.print_board(true);
            },

            "go" => {
                let depth = match args[0].parse::<i32>(){
                    Ok(depth) => depth,
                    Err(e) => {
                        println!("invalid depth: {}", e);
                        continue;
                    }
                };
                let start = std::time::Instant::now();
                let mov = get_best_move(&board.game, depth);
                println!("best move: {:?}", mov);
                println!("time: {:?}", start.elapsed());
                board.game.apply_move(&mov);
                board.game.print_board(false);



            }
            "allmoves" => {
                board.game.print_board(true);
                let moves = board.game.get_all_legal_moves().iter().for_each(|f| println!("{:?}", f));
            },
            _ => {
                println!("invalid command");
            }
 
        }


        
    }

    // CheckersBitboard::pretty_print_bitboard(generation::LOOKUP_TABLE.all_non_capturing_moves[0][40]);

}
