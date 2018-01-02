use iron::prelude::*;
use iron::status;
use iron::Handler;

use player::GlobalPlayer;

pub struct NextPlayerHandler {
    player: GlobalPlayer
}

impl NextPlayerHandler {
    pub fn new(player: GlobalPlayer) -> NextPlayerHandler {
        NextPlayerHandler {
            player
        }
    }
}

impl Handler for NextPlayerHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let mut player = self.player.lock().unwrap();
        player.next();

        Ok(Response::with((status::NoContent)))
    }
}