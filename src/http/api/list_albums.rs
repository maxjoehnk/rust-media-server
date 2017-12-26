use iron::prelude::*;
use iron::status;
use iron::Handler;

use std::sync::{Arc, Mutex};

use library::Library;

use serde_json;

pub struct ListAlbumsHandler {
    library: Arc<Mutex<Library>>
}

impl ListAlbumsHandler {
    pub fn new(library: Arc<Mutex<Library>>) -> ListAlbumsHandler {
        ListAlbumsHandler {
            library
        }
    }
}

impl Handler for ListAlbumsHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let library = self.library.lock().unwrap();
        let res = serde_json::to_string(&library.albums).unwrap();

        Ok(Response::with((mime!(Application/Json), status::Ok, res)))
    }
}