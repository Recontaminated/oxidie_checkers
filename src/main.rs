/*
    Javastyle docs. lets go

    this is the main driver for the checkers engine.
    It creates a new GameState and then loops recieving input from STDIO until the "endgame" command is recieved or the game is over.
    The engine can be interacted with using the PCI protocol
 */
use crate::{board::Move, algo::get_best_move};

// bring in all the modules into project scope
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
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

/*
Main driver function
 */
fn main() {
    // init items
    let transposition_table = &mut transposition::TranspositionTable::new();
    let mut board = board::GameState::starting_pos();
    println!("engine ready");
    board.game.print_board(false);


    //primary game loop
    loop {
        println!("side to move: {:?}", board.game.white_to_move);
    
        //get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: Vec<&str> = input.split_whitespace().collect();

        //validate and parse and handle input
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
            "movenum" => {
                
                if args.len() != 2 {
                    println!("invalid movenum command");
                    continue;
                }
                let from = match args[0].parse()
                {
                    Ok(from) => from,
                    Err(e) => {
                        println!("invalid from: {}", e);
                        continue;
                    }
                };
                 let to = match args[1].parse()
                {
                    Ok(to) => to,
                    Err(e) => {
                        println!("invalid to: {}", e);
                        continue;
                    }
                };

                board.game.move_piece(from, to);
                board.game.white_to_move = !board.game.white_to_move;
                board.game.print_board(false);
            }
            "move" => {
                let mut moves:Vec<Move> = Vec::new();
                //parse PCI move inpput notation into lsit of moves
                let mov = match Move::from_uci(Vec::from(args)) {
                    Ok(mov) => mov,
                    Err(e) => {
                        println!("invalid move: {}", e);
                        continue;
                    }
                };
                
                
                println!("move: {:?}", mov);
                let legal_moves = board.game.get_all_legal_moves();
                //check if we can actually make the move
                if !legal_moves.contains(&mov){
                    println!("invalid move");
                    continue;
                }
        
                //make the move and print the board
                board.game.apply_move(&mov);
                board.game.print_board(true);
            },
            //calls the engine to find the best move and makes it
            "go" => {
                match args[0] {
                    "depth" => {},
                    _ => {
                        println!("invalid go command");
                        continue;
                    }
                }
                let depth = match args[1].parse::<i32>(){
                    Ok(depth) => depth,
                    Err(e) => {
                        println!("invalid depth: {}", e);
                        continue;
                    }
                };
                let start = std::time::Instant::now();
                let mov = get_best_move(&board.game, depth, transposition_table);
                println!("cache hits: {}", transposition_table.cache_hits);
                println!("best move: {:?}", mov);
                println!("time: {:?}", start.elapsed());
                board.game.apply_move(&mov);
                board.game.white_to_move = !board.game.white_to_move;
                board.game.print_board(false);



            }
            //prints all legal moves
            "allmoves" => {
                board.game.print_board(true);
                let moves = board.game.get_all_legal_moves().iter().for_each(|f| println!("{:?}", f));
            },
            _ => {
                println!("invalid command");
            }
 
        }


        
    }


}
