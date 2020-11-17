pub const BOARD_SIZE: usize = 3;
pub const WIN_COMBINATION_SUM: i8 = 3;

use std::fmt;
pub enum TPieces {
    X,
    Y,
    EMPTY
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