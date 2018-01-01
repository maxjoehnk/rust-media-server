use iron::prelude::*;
use iron::status;
use iron::Handler;

use player::GlobalPlayer;

pub struct PausePlayerHandler {
    player: GlobalPlayer
}

impl PausePlayerHandler {
    pub fn new(player: GlobalPlayer) -> PausePlayerHandler {
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