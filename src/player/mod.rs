mod queue;
mod player;

use std::sync::{Arc, Mutex};

pub use self::queue::Queue;
pub use self::player::{Player, main_loop, PlayerState};

pub type SharedPlayer = Arc<Mutex<Player>>;