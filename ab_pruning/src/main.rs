//External dependencies
extern crate log;

#[path = "game/game.rs"] 
mod game;
#[path = "game/board_piece.rs"] 
mod piece;
#[path = "game/constants.rs"] 
mod constants;
#[path = "game/utility.rs"] 
mod utility;

use game::Game;
use log::{debug};

use crate::piece::Piece;
use crate::piece::Move;
use constants::GameImpl;

fn main() {
    //Initialize logger
    env_logger::init();
    let selected_game_type = GameImpl::TicTacToe;

    println!("You are playing :: {}", selected_game_type.value());

    debug!("Game Factory initialising");
    let mut selected_game = game::game_factory(selected_game_type);
    let mut board = selected_game.board_factory();
    
    let user = selected_game.user_side();
    game::play_game(&selected_game, &mut board, user);
}