use iron::prelude::*;
use iron::status;
use iron::Handler;

use player::SharedPlayer;

pub struct PlayPlayerHandler {
    player: SharedPlayer
}

impl PlayPlayerHandler {
    pub fn new(player: SharedPlayer) -> PlayPlayerHandler {
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