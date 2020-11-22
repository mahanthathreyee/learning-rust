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
    fn user_move(&self, board: &mut [Vec<Piece<Self::Piece>>], player_type: Player);
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

pub fn play_game<PieceType: Clone>(selected_game: &(impl Game + Game<Piece = PieceType>), board: &mut Vec<Vec<Piece<PieceType>>>, user: Player) {
    let (bot, mut is_bot_move) = match user {
        Player::ONE => (Player::TWO, false),
        Player::TWO => (Player::ONE, true),
        Player::DRAW => panic!("Invalid Player")
    };

    let (user_choice, bot_choice) = if is_bot_move { ("Y", "X") } else { ("X", "Y") };
    let player_choice = format!("USER :: {} \nBOT  :: {}\n", user_choice, bot_choice);

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", player_choice);
        selected_game.print_board(&board);

        if let Some(player) = selected_game.terminal_state(&board) {  
            match player {
                Player::ONE => println!("Player One has won the match"),
                Player::TWO => println!("Player Two has won the match"),
                Player::DRAW => println!("Match resulted in a draw")
            }
            break;
        }

        if is_bot_move {
            let new_bot_move = compute_bot_move(selected_game, board, bot.clone(), &mut None, 0);
            if let Some(bot_move) = new_bot_move { 
                selected_game.move_piece(bot.clone(), board, Move::new(bot_move.from, bot_move.to, 0));
            }
            is_bot_move = false;
        } else {
            selected_game.user_move(board, user.clone());
            is_bot_move = true;
        }
    }

}

//ASSUMPTIONS
//1. PLAYER ONE IS MAX 
//1. PLAYER TWO IS MAX 
fn compute_bot_move<PieceType: Clone>(selected_game: &(impl Game + Game<Piece = PieceType>), board: &mut Vec<Vec<Piece<PieceType>>>, current_player: Player, current_move: &mut Option<Move>, depth: u32) -> Option<Move> {
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
                            if new_move.value() > best_move.value() { new_move } else { best_move }
                        },
                        Player::TWO => {
                            //IF new_move is lesser then assign
                            if new_move.value() < best_move.value() { new_move } else { best_move }
                        },
                        Player::DRAW => best_move   //This case will never occur
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