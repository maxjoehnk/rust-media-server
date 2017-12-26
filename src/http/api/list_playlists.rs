use iron::prelude::*;
use iron::status;
use iron::Handler;

use std::sync::{Arc, Mutex};

use library::Library;

use serde_json;

pub struct ListPlaylistsHandler {
    library: Arc<Mutex<Library>>
}

impl ListPlaylistsHandler {
    pub fn new(library: Arc<Mutex<Library>>) -> ListPlaylistsHandler {
        ListPlaylistsHandler {
            library
        }
    }
}

impl Handler for ListPlaylistsHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let library = self.library.lock().unwrap();
        let res = serde_json::to_string(&library.playlists).unwrap();

        Ok(Response::with((mime!(Application/Json), status::Ok, res)))
    }
}