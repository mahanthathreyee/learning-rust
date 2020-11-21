#[path = "./constants.rs"] 
mod ttt_constants;

use crate::game::Game;
use crate::game::Piece;
use crate::constants::Player;
use crate::constants::GameImpl;
use crate::utility::read_user_input;


use ttt_constants::*;

use log::debug;
pub struct TicTacToeImpl {}

impl TicTacToeImpl {
    pub fn new() -> TicTacToeImpl {
        TicTacToeImpl {}
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

    fn user_move(&mut self, board: &mut [Vec<Piece<Self::Piece>>], playerType: Player) {
        let total_cells = BOARD_SIZE * BOARD_SIZE;
        loop {
            let input_move = read_user_input::<usize>() - 1;
            if input_move <= total_cells {
                let row = input_move / BOARD_SIZE;
                let col = input_move % BOARD_SIZE;
                
                if let TPieces::EMPTY = board[row][col].value() {
                    // board[row][col].update();
                    let new_piece = tpiece_by_player(playerType);
                    board[row][col].update(new_piece, new_piece.value().to_string());
                    break;
                }
            }
            println!("Position are already taken, please provide another input")
        }
    }

    fn successor_state<'board_life_time>(&self, board: &[Vec<Piece<Self::Piece>>]) -> Vec<([usize; 2], [usize; 2])> {
        let mut open_cells: Vec<([usize; 2], [usize; 2])> = Vec::new();
        for (row, board_row) in board.iter().enumerate() {
            for (col, board_col) in board_row.iter().enumerate() {
                if let TPieces::EMPTY = board_col.value() {
                    open_cells.push(([0, 0], [row, col]));
                }
            }
        }

        open_cells
    }

    fn terminal_state<'board_life_time>(&self, board: &[Vec<Piece<Self::Piece>>]) -> Option<Player> {
        let mut row_sum: [i8; BOARD_SIZE] = [0; BOARD_SIZE];
        let mut col_sum: [i8; BOARD_SIZE] = [0; BOARD_SIZE];
        let mut diag_sum: [i8; 2] = [0; 2];
        let mut empty_cell_count: u32 = 0;
        
        for (row, board_row) in board.iter().enumerate() {
            for (col, board_column) in board_row.iter().enumerate() {
                let value_of_cell: i8 = match board_column.value() {
                    TPieces::X => 1,
                    TPieces::Y => -1,
                    TPieces::EMPTY => {
                        empty_cell_count += 1; 
                        0
                    }
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

        let game_winner: Option<Player> = 
        if row_sum.contains(&WIN_COMBINATION_SUM)
            || col_sum.contains(&&WIN_COMBINATION_SUM)
            || diag_sum.contains(&WIN_COMBINATION_SUM) {
            Some(Player::ONE)
        } else if row_sum.contains(&(- WIN_COMBINATION_SUM))
            || col_sum.contains(&(- WIN_COMBINATION_SUM))
            || diag_sum.contains(&(- WIN_COMBINATION_SUM)) {
            Some(Player::TWO)
        } else if empty_cell_count > 0 {
            Some(Player::DRAW)
        } else {
            None
        };

        game_winner
    }

    fn move_piece(&self, player: Player, board: &mut [Vec<Piece<Self::Piece>>], _from: [usize; 2], to: [usize; 2]) {
        //FROM` is not needed in case TicTacToe, but is very useful in case of chess
        let player_piece = match player {
            Player::ONE => { TPieces::X }
            Player::TWO => { TPieces::Y }
            Player::DRAW => { TPieces::EMPTY }
        };
        board[to[0]][to[1]].update(player_piece.clone(), player_piece.value().to_string());
    }

    fn print_board(&self, board: &[Vec<Piece<Self::Piece>>]){
        for board_row in board {
            for board_column in board_row {
                print!("{} ", board_column.name());
            }
            println!();
        }
    }

    fn game_impl(&self) -> GameImpl {
        GameImpl::TicTacToe
    }
}