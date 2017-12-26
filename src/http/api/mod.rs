mod list_albums;
mod list_artists;
mod list_playlists;
mod list_tracks;

use std::sync::{Arc, Mutex};
use library::Library;
use router::Router;

use self::list_albums::ListAlbumsHandler;
use self::list_artists::ListArtistsHandler;
use self::list_playlists::ListPlaylistsHandler;
use self::list_tracks::ListTracksHandler;

pub fn build(library: Arc<Mutex<Library>>) -> Router {
    router!(
        list_albums:    get "/library/albums"    => ListAlbumsHandler::new(library.clone()),
        list_artists:   get "/library/artists"   => ListArtistsHandler::new(library.clone()),
        list_playlists: get "/library/playlists" => ListPlaylistsHandler::new(library.clone()),
        list_tracks:    get "/library/tracks"    => ListTracksHandler::new(library.clone())
    )
}