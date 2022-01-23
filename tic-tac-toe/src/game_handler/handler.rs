use crate::game_model::game::Game;
use crate::game_handler::tic_tac_toe::game_impl::TicTacToe;

pub fn start_new_game() {
    let mut game = TicTacToe::new();
    game.print_board();
    game.next_move();
    game.print_board();
    game.reset();
    game.print_board();
}