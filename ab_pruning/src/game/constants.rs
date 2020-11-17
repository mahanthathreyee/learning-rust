pub enum EndGameState {
    ONE, 
    TWO,
    DRAW
}

impl EndGameState {
    pub fn get_value(&self) -> i32 {
        match self {
            EndGameState::ONE => { -1 }
            EndGameState::DRAW => { 0 }
            EndGameState::TWO => { 1 }
        }
    }
}