#[path = "./game_impl/tic_tac_toe/tic_tac_toe_impl.rs"] 
mod tic_tac_toe;

use tic_tac_toe::TicTacToeImpl;
use crate::Piece;
use crate::GameImpl;

pub trait Game {
    fn initial_state(&mut self);
    fn successor_state(&self) -> Vec<u32>;
    fn terminal_state(&self) -> bool;
    fn utility_fn(&self);
    fn get_name(&self) -> &str;
    fn print_board(&self);
}

pub fn game_factory(game_impl: GameImpl) -> Box<dyn Game> {
    return match game_impl {
        GameImpl::TicTacToe => Box::new(TicTacToeImpl::new())
    };
}