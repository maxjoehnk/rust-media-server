use http::{HttpConfig, open};
use std::thread;
use app::SharedApp;

pub fn spawn(config: Option<HttpConfig>, app: SharedApp) -> thread::JoinHandle<()> {
    let config = config.unwrap_or(HttpConfig {
        ip: "0.0.0.0".to_owned(),
        port: 8080
    });
    thread::spawn(move|| {
        open(config, app).unwrap();
    })
}