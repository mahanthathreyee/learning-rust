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
use log::{info, debug};

use crate::piece::Piece;
use crate::piece::Move;
use constants::{GameImpl, Player};
use utility::read_user_input;

fn main() {
    //Initialize logger
    env_logger::init();
    let selected_game_type = GameImpl::TicTacToe;

    println!("You are playing :: {}", selected_game_type.value());

    debug!("Game Factory initialising");
    let mut selected_game = game::game_factory(selected_game_type);
    let mut board = selected_game.board_factory();
    
    loop {
        selected_game.print_board(&board);
        read_user_input::<u32>();

        game::compute_bot_move(&selected_game, &mut board, Player::ONE, &mut None);
    }

}