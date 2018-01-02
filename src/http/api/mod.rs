mod handler;
mod viewmodels;

use library::GlobalLibrary;
use player::GlobalPlayer;
use router::Router;
use self::handler::*;

pub fn build(player: GlobalPlayer, library: GlobalLibrary) -> Router {
    router!(
        list_albums:    get  "/library/albums"    => ListAlbumsHandler::new(library.clone()),
        list_artists:   get  "/library/artists"   => ListArtistsHandler::new(library.clone()),
        list_playlists: get  "/library/playlists" => ListPlaylistsHandler::new(library.clone()),
        list_tracks:    get  "/library/tracks"    => ListTracksHandler::new(library.clone()),
        pause:          post "/player/pause"      => PausePlayerHandler::new(player.clone())
    )
}