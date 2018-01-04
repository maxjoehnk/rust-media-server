use iron::prelude::*;
use iron::status;
use iron::Handler;
use router::Router;

use library::{SharedLibrary, Track};
use player::SharedPlayer;

pub struct AddToQueueHandler {
    player: SharedPlayer,
    library: SharedLibrary
}

impl AddToQueueHandler {
    pub fn new(player: SharedPlayer, library: SharedLibrary) -> AddToQueueHandler {
        AddToQueueHandler {
            player,
            library
        }
    }
}

impl Handler for AddToQueueHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
        let id = id.parse::<usize>().unwrap();
        let track: Option<Track> = self.library
            .get_track(&id);
        match track {
            Some(track) => {
                let mut player = self.player.lock().unwrap();
                player.queue.add_track(track);

                Ok(Response::with((status::NoContent)))
            },
            None => Ok(Response::with(status::NotFound))
        }
    }
}