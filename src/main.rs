#[macro_use]
extern crate slog;
extern crate slog_term;
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

mod mpd;
mod library;
mod player;
mod http;
mod provider;
mod jobs;

use provider::ProviderInstance;

use std::fs::File;
use std::io::prelude::*;

use std::thread;

use slog::Drain;

use std::sync::{Arc, Mutex};

lazy_static! {
    static ref logger: slog::Logger = slog::Logger::root(
        slog_term::FullFormat::new(slog_term::PlainSyncDecorator::new(std::io::stdout()))
            .build().fuse(), o!()
    );
}

#[derive(Deserialize, Clone)]
pub struct Config {
    mpd: Option<mpd::MpdConfig>,
    http: Option<http::HttpConfig>,
    pocketcasts: Option<provider::pocketcasts::PocketcastsProvider>,
    soundcloud: Option<provider::soundcloud::SoundcloudProvider>
}

fn testing(player: player::GlobalPlayer, library: library::GlobalLibrary) {
    let playlist = library::Playlist {
        title: "Test".to_owned(),
        tracks: vec![],
        provider: provider::Provider::LocalMedia
    };
    {
        let mut library = library.lock().unwrap();
        library.playlists.push(playlist);
    }

    {
        let library = library.lock().unwrap();
        let mut player = player.lock().unwrap();
        let tracks = library
            .search("Friendly Sessions");
        player.queue.add_multiple(tracks);
    }

    {
        let mut player = player.lock().unwrap();
        player.play();
    }
}

fn main() {
    gstreamer::init().unwrap();
    let mut config_file = File::open("config.toml").unwrap();
    let mut config = String::new();
    config_file.read_to_string(&mut config).unwrap();
    let config: Config = toml::from_str(config.as_str()).unwrap();

    let library = Arc::new(Mutex::new(library::Library::new()));
    let player = Arc::new(Mutex::new(player::Player::new()));

    let threads = vec![
        jobs::mpd::spawn(config.mpd.clone(), player.clone(), library.clone()),
        jobs::http::spawn(config.http.clone(), player.clone(), library.clone()),
        jobs::gst::spawn(player.clone()),
        jobs::sync::spawn(config.clone(), library.clone())
    ];

    testing(player.clone(), library.clone());

    for handle in threads {
        let _ = handle.join();
    }
}