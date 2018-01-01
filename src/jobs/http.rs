use http::{HttpConfig, open};
use std::thread;
use player::GlobalPlayer;
use library::GlobalLibrary;

pub fn spawn(config: Option<HttpConfig>, player: GlobalPlayer, library: GlobalLibrary) -> thread::JoinHandle<()> {
    let config = config.unwrap_or(HttpConfig {
        ip: "0.0.0.0".to_owned(),
        port: 8080
    });
    thread::spawn(move|| {
        open(config, player, library).unwrap();
    })
}