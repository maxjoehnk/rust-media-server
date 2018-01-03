use iron::prelude::*;
use iron::status;
use iron::Handler;

use library::GlobalLibrary;
use provider::SharedProviders;
use http::api::viewmodels::PlaylistModel;

use rayon::prelude::*;
use serde_json;

pub struct ListPlaylistsHandler {
    library: GlobalLibrary,
    providers: SharedProviders
}

impl ListPlaylistsHandler {
    pub fn new(library: GlobalLibrary, providers: SharedProviders) -> ListPlaylistsHandler {
        ListPlaylistsHandler {
            library,
            providers
        }
    }
}

impl Handler for ListPlaylistsHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let playlists: Vec<PlaylistModel> = self.library
            .playlists
            .read()
            .unwrap()
            .par_iter()
            .cloned()
            .map(|playlist| PlaylistModel::from(playlist, self.library.clone(), self.providers.clone()))
            .collect();
        let res = serde_json::to_string(&playlists).unwrap();

        Ok(Response::with((mime!(Application/Json), status::Ok, res)))
    }
}