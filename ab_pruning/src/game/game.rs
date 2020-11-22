#[path = "./game_impl/tic_tac_toe/tic_tac_toe_impl.rs"] 
mod tic_tac_toe;

use crate::Piece;
use crate::Move;
use crate::GameImpl;
use crate::constants::*;
use tic_tac_toe::TicTacToeImpl;

use log::debug;

pub trait Game {
    type Piece: Clone;
    
    fn board_factory(&mut self) -> Vec<Vec<Piece<Self::Piece>>>;
    fn user_side(&self) -> Player;
    fn user_move(&mut self, board: &mut [Vec<Piece<Self::Piece>>], player_type: Player);
    fn successor_state(&self, board: &[Vec<Piece<Self::Piece>>]) -> Vec<([usize; 2], [usize; 2])>;
    fn terminal_state(&self, board: &[Vec<Piece<Self::Piece>>]) -> Option<Player>;
    fn move_piece(&self, player: Player, board: &mut [Vec<Piece<Self::Piece>>], new_move: Move);
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
pub fn compute_bot_move<PieceType: Clone>(selected_game: &(impl Game + Game<Piece = PieceType>), board: &mut Vec<Vec<Piece<PieceType>>>, current_player: Player, current_move: &mut Option<Move>, depth: u32) -> Option<Move> {
    if let Some(current_move) = current_move {
        if let Some(player) = selected_game.terminal_state(board) {
            current_move.set_value(player.value());
            debug!("Depth:: {} | Winner :: {:?} | Move :: {:?}", depth, current_player, current_move);
            return Some(current_move.clone());
        }
    }

    let possible_moves = selected_game.successor_state(board);
    debug!("Depth:: {} | Current Player :: {:?} | Previous Move :: {:?} | Possible Moves :: {:?}", depth, current_player, current_move, possible_moves);
    let mut best_move: Option<Move> = None;

    let next_player = match current_player {
        Player::ONE => Player::TWO,
        Player::TWO => Player::ONE,
        Player::DRAW => Player::DRAW
    };

    for (from, to) in possible_moves {
        let mut child = board.clone();
        let new_move = Move::new(from, to, 0);
        selected_game.move_piece(current_player.clone(), &mut child, Move::new(from, to, 0));
        
        let new_move = compute_bot_move(selected_game, &mut child, next_player.clone(), &mut Some(new_move.clone()), depth + 1);
        best_move = Some(
            if let Some(new_move) = new_move {
                if let Some(best_move) = best_move {
                    match current_player {
                        Player::ONE => {
                            //IF new_move is greated then assign
                            if new_move.value() >= best_move.value() { new_move } else { best_move }
                        },
                        Player::TWO => {
                            //IF new_move is lesser then assign
                            if new_move.value() <= best_move.value() { new_move } else { best_move }
                        },
                        Player::DRAW => best_move
                    }
                } else {
                    new_move
                }
            } else {
                panic!("Invalid new move")
            }
        );
    }
    debug!("Depth:: {} | Winner :: {:?} | Best Move :: {:?}", depth, current_player, best_move);
    best_move    
}