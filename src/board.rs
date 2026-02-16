pub const SIZE: usize = 9;

#[derive(Debug, Clone)]
pub struct Board {
    cells: [[u8; SIZE]; SIZE],
}

impl Board {
    /// Creates a new empty Sudoku board.
    pub fn new() -> Self {
        Self {
            cells: [[0; SIZE]; SIZE],
        }
    }

    /// Gets the value at the given row and column.
    pub fn get_cell(&self, row: usize, col: usize) -> u8 {
        self.cells[row][col]
    }
}
