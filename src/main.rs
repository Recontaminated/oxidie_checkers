mod utils;
mod board;
mod generation;

fn main() {
    println!("Hello, world!");
    let mut board = board::GameState::startingPos();
    board.game.printBoard();
    board.game.move_piece(17, 26);
    board.game.printBoard();
    board.game.get_non_capture_moves(false).iter().for_each(|x| {
        println!("{:?} ", x);
    });
    // utils::pretty_print_bitboard(generation::LOOKUP_TABLE.all_capturing_moves[0][42]);
    while true {
        //ask for input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "exit" {
            break;
        }
        if input.trim() == "movesb" {
            board.game.get_all_captures(false).iter().for_each(|x| {
                println!("{:?} ", x);
            });
            board.game.printBoard();
            continue;
        }
        if input.trim() == "movesw" {
            board.game.get_all_captures(true).iter().for_each(|x| {
                println!("HOLY  ITS WORKING");
                println!("{:?} ", x);
            });
            board.game.printBoard();
            continue;
        }
        let input: Vec<&str> = input.split_whitespace().collect();
        let from = input[0].parse::<u8>().unwrap();
        let to = input[1].parse::<u8>().unwrap();
        board.game.move_piece(from, to);
        board.game.printBoard();
        board.game.get_non_capture_moves(false).iter().for_each(|x| {
            println!("{:?} ", x);
        });
        // board.game.get_captures(false).iter().for_each(|x| {
        //     println!("{:?} ", x);
        // });

        
    }

    // CheckersBitboard::pretty_print_bitboard(generation::LOOKUP_TABLE.all_non_capturing_moves[0][40]);

}
