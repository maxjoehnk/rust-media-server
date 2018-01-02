use iron::prelude::*;
use iron::status;
use iron::Handler;

use library::{GlobalLibrary, AlbumModel};

use serde_json;

pub struct ListAlbumsHandler {
    library: GlobalLibrary
}

impl ListAlbumsHandler {
    pub fn new(library: GlobalLibrary) -> ListAlbumsHandler {
        ListAlbumsHandler {
            library
        }
    }
}

impl Handler for ListAlbumsHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let albums: Vec<AlbumModel> = self.library
            .albums
            .read()
            .unwrap()
            .iter()
            .cloned()
            .map(|album| AlbumModel::from(album, self.library.clone()))
            .collect();
        let res = serde_json::to_string(&albums).unwrap();

        Ok(Response::with((mime!(Application/Json), status::Ok, res)))
    }
}