mod board;

fn main() {
    println!("Hello, world!");
    let mut board = board::GameState::startingPos();
    board.game.printBoard();
    board.movePiece(17, 26);
    board.game.printBoard();

    
}
