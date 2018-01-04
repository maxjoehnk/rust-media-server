#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate serde_mpd;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate cookie;
extern crate rayon;
extern crate toml;
extern crate gstreamer;
extern crate glib;
extern crate libc;
extern crate regex;
#[macro_use]
extern crate mime;
extern crate iron;
#[macro_use]
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate soundcloud;
extern crate url;

mod app;
mod bus;
mod mpd;
mod library;
mod player;
mod http;
mod provider;
mod jobs;
mod logger;

use std::fs::File;
use std::io::prelude::*;

use std::sync::{Arc, Mutex, RwLock};

#[derive(Deserialize, Clone)]
pub struct Config {
    mpd: Option<mpd::MpdConfig>,
    http: Option<http::HttpConfig>,
    pocketcasts: Option<provider::pocketcasts::PocketcastsProvider>,
    soundcloud: Option<provider::soundcloud::SoundcloudProvider>
}

fn read_config() -> Config {
    let mut config_file = File::open("config.toml").unwrap();
    let mut config = String::new();
    config_file.read_to_string(&mut config).unwrap();
    toml::from_str(config.as_str()).unwrap()
}

fn main() {
    gstreamer::init().unwrap();
    let config = read_config();
    let bus: bus::SharedBus = Arc::new(Mutex::new(bus::MessageBus::new()));
    let library: library::SharedLibrary = Arc::new(library::Library::new());
    let player: player::SharedPlayer = Arc::new(Mutex::new(player::Player::new(bus.clone())));

    let mut providers: provider::SharedProviders = vec![];
    {
        if config.pocketcasts.is_some() {
            providers.push(Arc::new(RwLock::new(Box::new(config.pocketcasts.unwrap()))));
        }
        if config.soundcloud.is_some() {
            providers.push(Arc::new(RwLock::new(Box::new(config.soundcloud.unwrap()))));
        }
    }

    let app = Arc::new(app::App {
        bus,
        player,
        library,
        providers
    });

    let threads = vec![
        jobs::mpd::spawn(config.mpd.clone(), app.clone()),
        jobs::http::spawn(config.http.clone(), app.clone()),
        jobs::gst::spawn(app.clone()),
        jobs::sync::spawn(app.clone())
    ];

    for handle in threads {
        let _ = handle.join();
    }
}