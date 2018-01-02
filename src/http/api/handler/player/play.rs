use iron::prelude::*;
use iron::status;
use iron::Handler;

use player::GlobalPlayer;

pub struct PlayPlayerHandler {
    player: GlobalPlayer
}

impl PlayPlayerHandler {
    pub fn new(player: GlobalPlayer) -> PlayPlayerHandler {
        PlayPlayerHandler {
            player
        }
    }
}

impl Handler for PlayPlayerHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let mut player = self.player.lock().unwrap();
        player.play();

        Ok(Response::with((status::NoContent)))
    }
}