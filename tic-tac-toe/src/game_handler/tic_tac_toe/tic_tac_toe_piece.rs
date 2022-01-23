use crate::game_model::piece::PieceValue;

pub struct TicTacToePiece {
    value: String
}

impl PieceValue<TicTacToePiece> for TicTacToePiece {
    fn new(value: String) -> TicTacToePiece {
        TicTacToePiece {
            value
        }
    }

    fn value(&self) -> String {
        self.value.clone()
    }

    fn update_value(&mut self,  new_value: String) {
        self.value = new_value;
    }
}