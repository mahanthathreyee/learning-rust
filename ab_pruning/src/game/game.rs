#[path = "./game_impl/tic_tac_toe/tic_tac_toe_impl.rs"] 
mod tic_tac_toe;

use crate::Piece;
use crate::Move;
use crate::GameImpl;
use crate::constants::*;

use tic_tac_toe::TicTacToeImpl;


pub trait Game {
    type Piece: Clone;
    
    fn board_factory(&mut self) -> Vec<Vec<Piece<Self::Piece>>>;
    fn user_move(&mut self, board: &mut [Vec<Piece<Self::Piece>>], playerType: Player);
    fn successor_state(&self, board: &[Vec<Piece<Self::Piece>>]) -> Vec<([usize; 2], [usize; 2])>;
    fn terminal_state(&self, board: &[Vec<Piece<Self::Piece>>]) -> Option<Player>;
    fn move_piece(&self, player: Player, board: &mut [Vec<Piece<Self::Piece>>], from: [usize; 2], to: [usize; 2]);
    fn game_impl(&self) -> GameImpl;
    fn print_board(&self, board: &[Vec<Piece<Self::Piece>>]);
}

pub fn game_factory(game_impl: GameImpl) -> impl Game {
    match game_impl {
        GameImpl::TicTacToe => TicTacToeImpl::new()
    }
}

//ASSUMPTIONS
//1. PLAYER ONE IS MAX 
//1. PLAYER TWO IS MAX 
pub fn compute_bot_move<PieceType: Clone>(selected_game: &(impl Game + Game<Piece = PieceType>), board: &mut Vec<Vec<Piece<PieceType>>>, current_player: Player, current_move: &mut Option<Move>) -> Move {
    if let Some(current_move) = current_move {
        if let Some(player) = selected_game.terminal_state(board) {
            current_move.set_value(player.value());
            return current_move.clone();
        }
    }

    let possible_moves = selected_game.successor_state(board);
    let mut best_move = Move::new(possible_moves[0].0, possible_moves[1].1, 0);

    for (from, to) in possible_moves {
        let mut child = board.clone();
        selected_game.move_piece(current_player.clone(), &mut child, from, to);
        let next_player = match current_player {
            Player::ONE => Player::TWO,
            Player::TWO => Player::ONE,
            Player::DRAW => Player::DRAW
        };
        
        let new_move = compute_bot_move(selected_game, &mut child, next_player, &mut Some(best_move.clone()));
        best_move = match current_player {
            Player::ONE => {
                //IF new_move is greated then assign
                if new_move.value() > best_move.value() { new_move } else { best_move }
            },
            Player::TWO => {
                //IF new_move is lesser then assign
                if new_move.value() < best_move.value() { new_move } else { best_move }
            },
            Player::DRAW => best_move
        }
    }

    best_move
}