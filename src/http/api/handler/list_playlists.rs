use iron::prelude::*;
use iron::status;
use iron::Handler;

use library::GlobalLibrary;
use http::api::viewmodels::PlaylistModel;

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
        let playlists: Vec<PlaylistModel> = self.library
            .playlists
            .read()
            .unwrap()
            .iter()
            .cloned()
            .map(|playlist| PlaylistModel::from(playlist, self.library.clone()))
            .collect();
        let res = serde_json::to_string(&playlists).unwrap();

        Ok(Response::with((mime!(Application/Json), status::Ok, res)))
    }
}