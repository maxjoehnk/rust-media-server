use iron::prelude::*;
use iron::status;
use iron::Handler;

use library::SharedLibrary;
use player::SharedPlayer;
use http::api::viewmodels::TrackModel;
use std::sync::Arc;

use serde_json;

pub struct GetQueueHandler {
    player: SharedPlayer,
    library: SharedLibrary
}

impl GetQueueHandler {
    pub fn new(player: SharedPlayer, library: SharedLibrary) -> GetQueueHandler {
        GetQueueHandler {
            player,
            library
        }
    }
}

impl Handler for GetQueueHandler {
    fn handle(&self, _req: &mut Request) -> IronResult<Response> {
        let player = self.player.lock().unwrap();
        let tracks: Vec<TrackModel> = player
            .queue
            .tracks
            .iter()
            .cloned()
            .map(|track| TrackModel::from(track, Arc::clone(&self.library)))
            .collect();

        let body = serde_json::to_string(&tracks).unwrap();

        Ok(Response::with((mime!(Application/Json), status::Ok, body)))
    }
}