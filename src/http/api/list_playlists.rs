use iron::prelude::*;
use iron::status;
use iron::Handler;

use library::GlobalLibrary;

use serde_json;

pub struct ListPlaylistsHandler {
    library: GlobalLibrary
}

impl ListPlaylistsHandler {
    pub fn new(library: GlobalLibrary) -> ListPlaylistsHandler {
        ListPlaylistsHandler {
            library
        }
    }
}

impl Handler for ListPlaylistsHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let res = serde_json::to_string(&self.library.playlists).unwrap();

        Ok(Response::with((mime!(Application/Json), status::Ok, res)))
    }
}