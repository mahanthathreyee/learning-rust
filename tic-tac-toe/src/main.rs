pub mod game_model;
pub mod util;
mod game_handler;

use game_handler::handler;

fn main() {
    handler::start_new_game();
}
