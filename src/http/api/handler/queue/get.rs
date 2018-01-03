use iron::prelude::*;
use iron::status;
use iron::Handler;

use library::GlobalLibrary;
use player::GlobalPlayer;
use http::api::viewmodels::TrackModel;

use serde_json;

pub struct GetQueueHandler {
    player: GlobalPlayer,
    library: GlobalLibrary
}

impl GetQueueHandler {
    pub fn new(player: GlobalPlayer, library: GlobalLibrary) -> GetQueueHandler {
        GetQueueHandler {
            player,
            library
        }
    }
}

impl Handler for GetQueueHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let player = self.player.lock().unwrap();
        let tracks: Vec<TrackModel> = player
            .queue
            .tracks
            .iter()
            .cloned()
            .map(|track| TrackModel::from(track, self.library.clone()))
            .collect();

        let body = serde_json::to_string(&tracks).unwrap();

        Ok(Response::with((mime!(Application/Json), status::Ok, body)))
    }
}