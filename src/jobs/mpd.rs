use mpd::{MpdConfig, open};
use std::thread;
use app::SharedApp;

pub fn spawn(config: Option<MpdConfig>, app: SharedApp) -> thread::JoinHandle<()> {
    let config = config.unwrap_or(MpdConfig {
        ip: "0.0.0.0".to_owned(),
        port: 6600
    });
    thread::spawn(move|| {
        open(config, app);
    })
}