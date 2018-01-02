use iron::prelude::*;
use iron::status;
use iron::Handler;

use serde_json;

use library::GlobalLibrary;
use player::{GlobalPlayer, PlayerState};
use http::api::viewmodels::{PlayerModel, TrackModel};

pub struct PlayerStateHandler {
    player: GlobalPlayer,
    library: GlobalLibrary
}

impl PlayerStateHandler {
    pub fn new(player: GlobalPlayer, library: GlobalLibrary) -> PlayerStateHandler {
        PlayerStateHandler {
            player,
            library
        }
    }
}

impl Handler for PlayerStateHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let mut player = self.player.lock().unwrap();
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