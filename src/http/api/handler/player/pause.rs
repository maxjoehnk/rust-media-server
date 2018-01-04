use iron::prelude::*;
use iron::status;
use iron::Handler;

use player::SharedPlayer;

pub struct PausePlayerHandler {
    player: SharedPlayer
}

impl PausePlayerHandler {
    pub fn new(player: SharedPlayer) -> PausePlayerHandler {
        PausePlayerHandler {
            player
        }
    }
}

impl Handler for PausePlayerHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let mut player = self.player.lock().unwrap();
        player.pause();

        Ok(Response::with((status::NoContent)))
    }
}