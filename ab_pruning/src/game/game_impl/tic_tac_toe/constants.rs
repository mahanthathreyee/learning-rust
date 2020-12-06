pub const BOARD_SIZE: usize = 3;
pub const WIN_COMBINATION_SUM: i8 = 3;

use std::fmt;

use crate::constants::Player;

#[derive(Clone)]
pub enum TPieces {
    X,
    Y,
    EMPTY
}

impl TPieces {
    pub fn value(&self) -> &str {
        match *self {
            TPieces::X => { "X" }
            TPieces::Y => { "Y" }
            TPieces::EMPTY => { "" }
        }
    }
}

pub fn tpiece_by_player(player: Player) -> TPieces {
    match player {
        Player::ONE => { TPieces::X }
        Player::TWO => { TPieces::Y } 
        Player::DRAW => { TPieces::EMPTY }
    }
}

impl fmt::Display for TPieces {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TPieces::X => write!(f, "X"),
            TPieces::Y => write!(f, "Y"),
            TPieces::EMPTY => write!(f, "Y")
        }
     }
}