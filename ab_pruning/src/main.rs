//External dependencies
extern crate log;

#[path = "game/game.rs"] 
mod game;
#[path = "game/board_piece.rs"] 
mod piece;

use crate::piece::Piece;

pub enum GameImpl {
    TicTacToe
}

fn main() {
    //Initialize logger
    env_logger::init();

    let selected_game_type = GameImpl::TicTacToe;

    let mut selected_game = game::game_factory(selected_game_type);
    selected_game.initial_state();
    println!("Initial Board State");
    selected_game.print_board();
}