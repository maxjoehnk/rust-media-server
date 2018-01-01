use player::{GlobalPlayer, main_loop};
use std::thread;

pub fn spawn(player: GlobalPlayer) -> thread::JoinHandle<()> {
    main_loop(player)
}