pub enum GameImpl {
    TicTacToe
}

impl GameImpl {
    pub fn value(&self) -> &str {
        match self {
            GameImpl::TicTacToe => { "Tic Tac Toe" }
        }
    }
}

#[derive(Clone)]
pub enum Player {
    ONE, 
    TWO,
    DRAW
}

impl Player {
    pub fn value(&self) -> i32 {
        match self {
            Player::ONE => { -1 }
            Player::DRAW => { 0 }
            Player::TWO => { 1 }
        }
    }
}