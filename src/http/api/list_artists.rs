use iron::prelude::*;
use iron::status;
use iron::Handler;

use library::GlobalLibrary;

use serde_json;

pub struct ListArtistsHandler {
    library: GlobalLibrary
}

impl ListArtistsHandler  {
    pub fn new(library: GlobalLibrary) -> ListArtistsHandler  {
        ListArtistsHandler  {
            library
        }
    }
}

impl Handler for ListArtistsHandler  {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let res = serde_json::to_string(&self.library.artists).unwrap();

        Ok(Response::with((mime!(Application/Json), status::Ok, res)))
    }
}