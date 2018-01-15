use player::main_loop;
use app::SharedApp;
use std::thread;
use std::sync::Arc;

pub fn spawn(app: SharedApp) -> thread::JoinHandle<()> {
    let player = Arc::clone(&app.player);
    main_loop(player)
}