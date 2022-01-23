#[derive(Clone)]
pub struct Piece<V: PieceValue<V>> {
    pub value: V,
    pub player: Player
}

pub trait PieceValue<V> {
    fn new(value: String) -> V;
    fn value(&self) -> String;
    fn update_value(&mut self, new_value: String);
}

#[derive(Copy, Clone)]
pub enum Player {
    ONE, TWO, NONE
}

impl <V: PieceValue<V>> Piece<V> {
    pub fn new(value: V) -> Piece<V> {
        Piece {
            value,
            player: Player::NONE
        }
    }
}