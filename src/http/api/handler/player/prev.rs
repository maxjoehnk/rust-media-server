use iron::prelude::*;
use iron::status;
use iron::Handler;

use player::GlobalPlayer;

pub struct PrevPlayerHandler {
    player: GlobalPlayer
}

impl PrevPlayerHandler {
    pub fn new(player: GlobalPlayer) -> PrevPlayerHandler {
        PrevPlayerHandler {
            player
        }
    }
}

impl Handler for PrevPlayerHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let mut player = self.player.lock().unwrap();
        player.prev();

        Ok(Response::with((status::NoContent)))
    }
}