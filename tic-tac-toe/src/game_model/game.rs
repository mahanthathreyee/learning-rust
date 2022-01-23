pub trait Game {
    fn print_board(&self);
    fn reset(&mut self);
    fn next_move(&mut self);
}