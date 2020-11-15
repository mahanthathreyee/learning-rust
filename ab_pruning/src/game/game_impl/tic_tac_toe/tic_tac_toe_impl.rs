#[path = "./constants.rs"] 
mod constants;

use crate::game::Game;
use crate::game::Piece;

use constants::*;

use log::debug;

pub struct TicTacToeImpl {
    board: Vec<Vec<Piece<TPieces>>>
}

impl TicTacToeImpl {
    pub fn new() -> TicTacToeImpl {
        TicTacToeImpl {
            board: Vec::new()
        }
    }
}

impl Game for TicTacToeImpl {
    fn initial_state(&mut self) {
        debug!("Game choosen :: Tic Tac Toe");
        debug!("Initializing the board");
        for row in 0..BOARD_SIZE {
            let mut board_row: Vec<Piece<TPieces>> = Vec::new();
            for column in 0..BOARD_SIZE {
                let index = (row * BOARD_SIZE) + column + 1;
                board_row.push(Piece::new(TPieces::EMPTY, index as u32, (index + 1).to_string()))
            }
            self.board.push(board_row);
        }
    }

    fn successor_state(&self) -> Vec<u32> {
        let mut open_cells: Vec<u32> = Vec::new();
        for board_cell in self.board.iter().flatten() {
            match board_cell.clone().get_value() {
                TPieces::EMPTY => {
                    open_cells.push(board_cell.clone().get_location())
                }
                TPieces::X => {}
                TPieces::Y => {}
            }
        }

        open_cells
    }

    fn terminal_state(&self) -> bool {
        let mut board_filled: bool = true;
        for board_cell in self.board.iter().flatten() {
            match board_cell.clone().get_value() {
                TPieces::EMPTY => {
                    board_filled = false; 
                    break;
                },
                TPieces::X => {}
                TPieces::Y => {}
            }
        }
        board_filled
    }

    fn utility_fn(&self) {
        
    }
    
    fn get_name(&self) -> &str { "Tic Tac Toe" }

    fn print_board(&self) {
        for board_row in self.board.clone() {
            for board_column in board_row {
                print!("{} ", board_column.get_name());
            }
            print!("\n");
        }
    }
}