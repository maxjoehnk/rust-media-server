use http::{HttpConfig, open};
use std::thread;
use player::GlobalPlayer;
use library::GlobalLibrary;
use provider::SharedProviders;

pub fn spawn(config: Option<HttpConfig>, player: GlobalPlayer, library: GlobalLibrary, providers: SharedProviders) -> thread::JoinHandle<()> {
    let config = config.unwrap_or(HttpConfig {
        ip: "0.0.0.0".to_owned(),
        port: 8080
    });
    thread::spawn(move|| {
        open(config, player, library, providers).unwrap();
    })
}