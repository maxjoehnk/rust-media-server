mod handler;
mod viewmodels;

use library::GlobalLibrary;
use player::GlobalPlayer;
use provider::SharedProviders;
use router::Router;
use self::handler::*;

pub fn build(player: GlobalPlayer, library: GlobalLibrary, providers: SharedProviders) -> Router {
    router!(
        list_albums:    get  "/library/albums"     => ListAlbumsHandler::new(library.clone()),
        get_album:      get  "/library/albums/:id" => GetAlbumHandler::new(library.clone()),
        list_artists:   get  "/library/artists"    => ListArtistsHandler::new(library.clone()),
        list_playlists: get  "/library/playlists"  => ListPlaylistsHandler::new(library.clone(), providers.clone()),
        list_tracks:    get  "/library/tracks"     => ListTracksHandler::new(library.clone()),
        pause:          post "/player/pause"       => player::PausePlayerHandler::new(player.clone()),
        play:           post "/player/play"        => player::PlayPlayerHandler::new(player.clone()),
        next:           post "/player/next"        => player::NextPlayerHandler::new(player.clone()),
        prev:           post "/player/prev"        => player::PrevPlayerHandler::new(player.clone()),
        player_state:   get  "/player"             => player::PlayerStateHandler::new(player.clone(), library.clone()),
        add_to_queue:   post "/queue/:id"          => queue::AddToQueueHandler::new(player.clone(), library.clone()),
        get_queue:      get  "/queue"              => queue::GetQueueHandler::new(player.clone(), library.clone())
    )
}