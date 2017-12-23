#[macro_use]
extern crate slog;
extern crate slog_term;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_json;
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


mod mpd;
mod pocketcasts;
mod library;
mod player;

use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;

use std::thread;

use slog::Drain;

use pocketcasts::PocketcastUser;
use rayon::iter::FromParallelIterator;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref logger: slog::Logger = slog::Logger::root(
        slog_term::FullFormat::new(slog_term::PlainSyncDecorator::new(std::io::stdout()))
            .build().fuse(), o!()
    );
}

#[derive(Deserialize, Clone)]
struct Config {
    pocketcasts: PocketcastUser,
    mpd: mpd::MpdConfig
}

fn main() {
    gstreamer::init().unwrap();
    let mut config_file = File::open("config.toml").unwrap();
    let mut config = String::new();
    config_file.read_to_string(&mut config);
    let config: Config = toml::from_str(config.as_str()).unwrap();

    let mut library = Arc::new(Mutex::new(library::Library::new()));

    let mut podcasts = config.pocketcasts.get_subscriptions().unwrap();
    let mut episodes: Vec<library::Track> = podcasts
        .par_iter_mut()
        .map(|podcast| {
            podcast.get_episodes(&config.pocketcasts).unwrap()
        })
        .reduce(|| vec![], |mut a, mut b| {
            a.append(&mut b);
            a
        })
        .iter()
        .map(|episode| episode.to_track())
        .collect();
    {
        let mut library = library.lock().unwrap();
        library.add_all(&mut episodes);
    }

    let mut player = player::Player::new();

    let mpd_config = config.mpd.clone();

    let mut mpd_player = player.clone();
    let mpd_library = library.clone();

    thread::spawn(move|| {
        mpd::open(mpd_config, mpd_player, mpd_library);
    });

    let playlist = library::Playlist {
        title: "Test".to_owned(),
        tracks: vec![]
    };
    {
        let mut library = library.lock().unwrap();
        library.playlists.push(playlist);
    }

    {
        let library = library.lock().unwrap();
        let tracks = library
            .search("Friendly Sessions");
        player.queue.add_multiple(tracks);
    }

    player.play();
}
