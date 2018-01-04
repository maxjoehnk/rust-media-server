use iron::prelude::*;
use iron::status;
use iron::Handler;

use serde_json;

use library::SharedLibrary;
use player::{SharedPlayer, PlayerState};
use http::api::viewmodels::{PlayerModel, TrackModel};

pub struct PlayerStateHandler {
    player: SharedPlayer,
    library: SharedLibrary
}

impl PlayerStateHandler {
    pub fn new(player: SharedPlayer, library: SharedLibrary) -> PlayerStateHandler {
        PlayerStateHandler {
            player,
            library
        }
    }
}

impl Handler for PlayerStateHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let player = self.player.lock().unwrap();
        let current = player.queue
            .current()
            .cloned()
            .map(|track| TrackModel::from(track, self.library.clone()));

        let state = PlayerModel {
            playing: (player.state == PlayerState::Play),
            current
        };

        let body = serde_json::to_string(&state).unwrap();

        Ok(Response::with((mime!(Application/Json), status::Ok, body)))
    }
}