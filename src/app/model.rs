pub mod song;
pub mod player;
pub mod library;

use player::Player;
use library::Library;

#[derive(Debug)]
pub struct Model {
    player: Player,
    library: Library,
}
