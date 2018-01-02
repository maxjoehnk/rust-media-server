use iron::prelude::*;
use iron::status;
use iron::Handler;

use library::GlobalLibrary;
use http::api::viewmodels::ArtistModel;

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
        let artists: Vec<ArtistModel> = self.library
            .artists
            .read()
            .unwrap()
            .iter()
            .cloned()
            .map(|artist| ArtistModel::from(artist, self.library.clone()))
            .collect();
        let res = serde_json::to_string(&artists).unwrap();

        Ok(Response::with((mime!(Application/Json), status::Ok, res)))
    }
}