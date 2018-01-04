use player::main_loop;
use app::SharedApp;
use std::thread;

pub fn spawn(app: SharedApp) -> thread::JoinHandle<()> {
    let player = app.player.clone();
    main_loop(player)
}