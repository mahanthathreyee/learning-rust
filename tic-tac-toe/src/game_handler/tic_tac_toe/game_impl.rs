use crate::game_model::game::Game;
use crate::game_model::board::Board;
use super::tic_tac_toe_piece::TicTacToePiece;
use super::constants as Constants;
use crate::util;

pub struct TicTacToe {
    board: Board<TicTacToePiece>
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe {
            board: Board::<TicTacToePiece>::new(Constants::BOARD_ROWS, Constants::BOARD_COLS)
        }
    }
}

impl Game for TicTacToe {
    fn print_board(&self) {
        self.board.print_board();
    }

    fn reset(&mut self) {
        self.board.reset();
    }

    fn next_move(&mut self) {
        print!("\nSelect cell: ");
        let user_input = util::read_user_input::<usize>();
        self.board.update_value("X".to_owned(), user_input);
    }
}