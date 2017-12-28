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
#[macro_use]
extern crate mime;
extern crate iron;
#[macro_use]
extern crate router;
extern crate mount;
extern crate staticfile;

mod mpd;
mod pocketcasts;
mod library;
mod player;
mod http;

use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;

use std::thread;

use slog::Drain;

use pocketcasts::PocketcastUser;
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
    mpd: mpd::MpdConfig,
    http: http::HttpConfig
}

fn main() {
    gstreamer::init().unwrap();
    let mut config_file = File::open("config.toml").unwrap();
    let mut config = String::new();
    config_file.read_to_string(&mut config).unwrap();
    let config: Config = toml::from_str(config.as_str()).unwrap();

    let library = Arc::new(Mutex::new(library::Library::new()));
    {
        let mut library = library.lock().unwrap();
        sync_pocketcasts(&config.pocketcasts, &mut library);
    }

    let mut player = player::Player::new();

    {
        let config = config.mpd.clone();

        let player = player.clone();
        let library = library.clone();

        thread::spawn(move|| {
            mpd::open(config, player, library);
        });
    }

    {
        let config = config.http.clone();
        let library = library.clone();
        thread::spawn(move|| {
            http::open(config, library).unwrap();
        });
    }

    let playlist = library::Playlist {
        title: "Test".to_owned(),
        tracks: vec![],
        provider: library::Provider::LocalMedia
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

fn sync_pocketcasts(user: &PocketcastUser, library: &mut library::Library) {
    let mut podcasts = user.get_subscriptions().unwrap();
    let mut episodes: Vec<library::Track> = podcasts
        .par_iter_mut()
        .map(|podcast| {
            podcast.get_episodes(&user).unwrap()
        })
        .reduce(|| vec![], |mut a, mut b| {
            a.append(&mut b);
            a
        })
        .iter()
        .map(|episode| episode.to_track())
        .collect();
    library.add_tracks(&mut episodes);
    let mut albums = podcasts.par_iter()
        .map(|podcast| podcast.to_album())
        .collect();
    library.add_albums(&mut albums);
}