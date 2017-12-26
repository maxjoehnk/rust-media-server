use iron::prelude::*;
use iron::status;
use iron::Handler;

use std::sync::{Arc, Mutex};

use library::Library;

use serde_json;

pub struct ListTracksHandler {
    library: Arc<Mutex<Library>>
}

impl ListTracksHandler {
    pub fn new(library: Arc<Mutex<Library>>) -> ListTracksHandler {
        ListTracksHandler {
            library
        }
    }
}

impl Handler for ListTracksHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let library = self.library.lock().unwrap();
        let res = serde_json::to_string(&library.tracks).unwrap();

        Ok(Response::with((mime!(Application/Json), status::Ok, res)))
    }
}