#[path = "./game_impl/tic_tac_toe/tic_tac_toe_impl.rs"] 
mod tic_tac_toe;

use tic_tac_toe::TicTacToeImpl;
use crate::Piece;
use crate::GameImpl;
use crate::constants;

pub trait Game {
    type Piece;
    
    fn board_factory(&mut self) -> Vec<Vec<Piece<Self::Piece>>>;
    fn successor_state(&self, board: &[Vec<Piece<Self::Piece>>]) -> Vec<[usize; 2]>;
    fn terminal_state(&self, board: &[Vec<Piece<Self::Piece>>]) -> constants::EndGameState;
    fn utility_fn(&self) -> i32;
    fn get_name(&self) -> &str;
    fn print_board(&self, board: &[Vec<Piece<Self::Piece>>]);
}

pub fn game_factory(game_impl: GameImpl) -> impl Game {
    match game_impl {
        GameImpl::TicTacToe => TicTacToeImpl::new()
    }
}