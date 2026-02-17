mod board;

use board::Board;

fn main() {
    let mut board = Board::new();
    
    // Set a few values for demonstration
    board.set_cell(0, 0, 5);
    board.set_cell(0, 1, 3);
    board.set_cell(4, 4, 8);
    board.set_cell(8, 8, 9);

    println!("Sudoku Board:");
    println!("{}", board);
}
