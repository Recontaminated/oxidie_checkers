mod board;

fn main() {
    println!("Hello, world!");
    let mut board = board::GameState::startingPos();
    board.game.printBoard();
    board.movePiece(17, 26);
    board.game.printBoard();
    board.game.get_jumps(true);

    let moves = board::CheckersBitboard::pregen_moves();

    println!("Move: {:064b}", moves[1][19]);
    board::CheckersBitboard::pretty_print_bitboard(moves[1][19]);

    
}
