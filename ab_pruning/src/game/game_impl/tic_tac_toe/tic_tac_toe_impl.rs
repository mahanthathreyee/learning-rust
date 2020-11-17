#[path = "./constants.rs"] 
mod ttt_constants;

use crate::game::Game;
use crate::game::Piece;
use crate::constants::EndGameState;

use ttt_constants::*;

use log::debug;

pub struct TicTacToeImpl {
    depth: i32
}

impl TicTacToeImpl {
    pub fn new() -> TicTacToeImpl {
        TicTacToeImpl {
            depth: 0
        }
    }
}

impl Game for TicTacToeImpl {
    type Piece = TPieces;

    fn board_factory(&mut self) -> Vec<Vec<Piece<Self::Piece>>> {
        debug!("Game choosen :: Tic Tac Toe");
        debug!("Initializing the board");
        let mut board: Vec<Vec<Piece<TPieces>>> = Vec::new();
        for row in 0..BOARD_SIZE {
            let mut board_row: Vec<Piece<TPieces>> = Vec::new();
            for column in 0..BOARD_SIZE {
                let index = (row * BOARD_SIZE) + column;
                board_row.push(Piece::new(TPieces::EMPTY, index as u32, (index + 1).to_string()))
            }
            board.push(board_row);
        }
        board
    }

    fn successor_state<'board_life_time>(&self, board: &[Vec<Piece<Self::Piece>>]) -> Vec<[usize; 2]> {
        let mut open_cells: Vec<[usize; 2]> = Vec::new();
        for (row, board_row) in board.iter().enumerate() {
            for (col, board_col) in board_row.iter().enumerate() {
                match board_col.get_value() {
                    TPieces::EMPTY => {
                        open_cells.push([row, col]);
                    }
                    TPieces::X => {}
                    TPieces::Y => {}
                }
            }
        }

        open_cells
    }

    fn terminal_state<'board_life_time>(&self, board: &[Vec<Piece<Self::Piece>>]) -> EndGameState {
        let mut row_sum: [i8; BOARD_SIZE] = [0; BOARD_SIZE];
        let mut col_sum: [i8; BOARD_SIZE] = [0; BOARD_SIZE];
        let mut diag_sum: [i8; 2] = [0; 2];
        
        for (row, board_row) in board.iter().enumerate() {
            for (col, board_column) in board_row.iter().enumerate() {
                let value_of_cell: i8 = match board_column.get_value() {
                    TPieces::X => 1,
                    TPieces::Y => -1,
                    TPieces::EMPTY => 0
                };

                if row == col {
                    diag_sum[0] += value_of_cell;
                }
                if row + col == (BOARD_SIZE - 1) {
                    diag_sum[1] += value_of_cell;
                }
                row_sum[row] += value_of_cell;
                col_sum[col] += value_of_cell;
            }
        }

        let game_winner: EndGameState = 
        if row_sum.contains(&WIN_COMBINATION_SUM)
            || col_sum.contains(&&WIN_COMBINATION_SUM)
            || diag_sum.contains(&WIN_COMBINATION_SUM) {
            EndGameState::ONE
        } else if row_sum.contains(&(- WIN_COMBINATION_SUM))
            || col_sum.contains(&(- WIN_COMBINATION_SUM))
            || diag_sum.contains(&(- WIN_COMBINATION_SUM)) {
            EndGameState::TWO
        } else {
            EndGameState::DRAW
        };

        game_winner
    }

    fn utility_fn(&self) -> i32 {
        1
    }
    
    fn get_name(&self) -> &str { "Tic Tac Toe" }

    fn print_board(&self, board: &[Vec<Piece<Self::Piece>>]){
        for board_row in board {
            for board_column in board_row {
                print!("{} ", board_column.get_name());
            }
            println!();
        }
    }
}