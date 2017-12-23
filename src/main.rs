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

use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;

use slog::Drain;

lazy_static! {
    static ref logger: slog::Logger = slog::Logger::root(
        slog_term::FullFormat::new(slog_term::PlainSyncDecorator::new(std::io::stdout()))
            .build().fuse(), o!()
    );
}

//mod mpd;
mod pocketcasts;
mod library;
mod player;

use pocketcasts::PocketcastUser;
use rayon::iter::FromParallelIterator;

#[derive(Deserialize)]
struct Config {
    pocketcasts: PocketcastUser
}

fn main() {
    gstreamer::init().unwrap();
    let mut config_file = File::open("config.toml").unwrap();
    let mut config = String::new();
    config_file.read_to_string(&mut config);
    let config: Config = toml::from_str(config.as_str()).unwrap();

    let mut library = library::Library::new();

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
    library.add_all(&mut episodes);

    let mut player = player::Player::new();
    let tracks = library
        .tracks
        .into_iter()
        .take(5)
        .collect();
    player.queue.add_multiple(tracks);
    //player.play();

    // mpd::open("0.0.0.0:6600");
}
