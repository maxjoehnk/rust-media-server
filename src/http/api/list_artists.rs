use iron::prelude::*;
use iron::status;
use iron::Handler;

use std::sync::{Arc, Mutex};

use library::Library;

use serde_json;

pub struct ListArtistsHandler {
    library: Arc<Mutex<Library>>
}

impl ListArtistsHandler  {
    pub fn new(library: Arc<Mutex<Library>>) -> ListArtistsHandler  {
        ListArtistsHandler  {
            library
        }
    }
}

impl Handler for ListArtistsHandler  {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let library = self.library.lock().unwrap();
        let res = serde_json::to_string(&library.artists).unwrap();

        Ok(Response::with((mime!(Application/Json), status::Ok, res)))
    }
}