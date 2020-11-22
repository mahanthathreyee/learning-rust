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
use constants::{GameImpl, Player};

fn main() {
    //Initialize logger
    env_logger::init();
    let selected_game_type = GameImpl::TicTacToe;

    println!("You are playing :: {}", selected_game_type.value());

    debug!("Game Factory initialising");
    let mut selected_game = game::game_factory(selected_game_type);
    let mut board = selected_game.board_factory();
    
    let user_player_type = selected_game.user_side();
    let bot_player_type = if let Player::ONE = user_player_type { Player::TWO } else {  Player::ONE };

    loop {
        selected_game.print_board(&board);
        
        selected_game.user_move(&mut board, user_player_type.clone());
        selected_game.print_board(&board);
        if let Some(player) = selected_game.terminal_state(&board) {
            victory(player);    break;
        }

        println!("Computing bot move");
        let bot_move = game::compute_bot_move(&selected_game, &mut board, bot_player_type.clone(), &mut None, 0);
        if let Some(bot_move) = bot_move { 
            selected_game.move_piece(bot_player_type.clone(), &mut board, Move::new(bot_move.from, bot_move.to, 0));
        }
        if let Some(player) = selected_game.terminal_state(&board) { 
            victory(player); break; 
        }
    }

}

fn victory(winner: Player) {
    match winner {
        Player::ONE => println!("Player One has won the match"),
        Player::TWO => println!("Player Two has won the match"),
        Player::DRAW => println!("Match resulted in a draw")
    }
}