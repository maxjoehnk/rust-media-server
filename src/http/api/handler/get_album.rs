use iron::prelude::*;
use iron::status;
use iron::Handler;
use router::Router;

use library::SharedLibrary;
use http::api::viewmodels::AlbumModel;

use serde_json;

pub struct GetAlbumHandler {
    library: SharedLibrary
}

impl GetAlbumHandler {
    pub fn new(library: SharedLibrary) -> GetAlbumHandler {
        GetAlbumHandler {
            library
        }
    }
}

impl Handler for GetAlbumHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
        let id = id.parse::<usize>().unwrap();
        let album: Option<AlbumModel> = self.library
            .get_album(&id)
            .map(|album| AlbumModel::from(album, self.library.clone()));
        match album {
            Some(album) => {
                let res = serde_json::to_string(&album).unwrap();

                Ok(Response::with((mime!(Application/Json), status::Ok, res)))
            },
            None => Ok(Response::with(status::NotFound))
        }
    }
}