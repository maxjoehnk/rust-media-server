use mpd::{MpdConfig, open};
use std::thread;
use player::GlobalPlayer;
use library::GlobalLibrary;
use provider::SharedProviders;

pub fn spawn(config: Option<MpdConfig>, player: GlobalPlayer, library: GlobalLibrary, providers: SharedProviders) -> thread::JoinHandle<()> {
    let config = config.unwrap_or(MpdConfig {
        ip: "0.0.0.0".to_owned(),
        port: 6600
    });
    thread::spawn(move|| {
        open(config, player, library, providers);
    })
}