use super::piece::Piece;
use super::piece::PieceValue;

pub struct Board<V: PieceValue<V>> {
    pub row_count: usize,
    pub col_count: usize,
    board: Vec<Vec<Piece<V>>>
}

impl <V: PieceValue<V>> Board<V> {
    pub fn new(row_count: usize, col_count: usize) -> Board<V> {
        Board {
            row_count,
            col_count,
            board: Board::construct_board(row_count, col_count)
        }
    }

    pub fn reset(&mut self) {
        self.board = Board::construct_board(self.row_count, self.col_count);
    }

    pub fn update_value(&mut self, new_value: String, input_position: usize) {
        match self.get_xy_from_position(input_position) {
            Some(position) => {
                self.board[position.0][position.1].value.update_value(new_value);
            }
            _ => {
                panic!("Unknown position");
            }
        }

    }

    pub fn print_board(&self) {
        for row_index in 0..self.row_count {
            for col_index in 0..self.col_count {
                let piece = &self.board[row_index][col_index].value;
                print!("{} ", piece.value())
            }
            println!();
        }
    }

    // Private functions
    fn construct_board(row_count: usize, col_count: usize) -> Vec<Vec<Piece<V>>> {
        let mut board: Vec<Vec<Piece<V>>> = Vec::new();
        for row_index in 0..row_count {
            let mut row = Vec::<Piece<V>>::new();
            for col_index in 0..col_count {
                let position: usize = (col_count * row_index) + col_index + 1;
                row.push(Piece::new(V::new(position.to_string())));
            }
            board.push(row);
        }

        board
    }

    fn get_xy_from_position(&self, input_position: usize) -> Option::<(usize, usize)> {
        let mut xy: Option::<(usize, usize)> = Option::None;
        for row_index in 0..self.row_count {
            for col_index in 0..self.col_count {
                let position: usize = (self.col_count * row_index) + col_index + 1;
                if position == input_position {
                    xy = Some((row_index, col_index));
                    break;
                }
            }
        }

        xy
    }
}