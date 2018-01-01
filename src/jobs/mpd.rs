use mpd::{MpdConfig, open};
use std::thread;
use player::GlobalPlayer;
use library::GlobalLibrary;

pub fn spawn(config: Option<MpdConfig>, player: GlobalPlayer, library: GlobalLibrary) -> thread::JoinHandle<()> {
    let config = config.unwrap_or(MpdConfig {
        ip: "0.0.0.0".to_owned(),
        port: 6600
    });
    thread::spawn(move|| {
        open(config, player, library);
    })
}