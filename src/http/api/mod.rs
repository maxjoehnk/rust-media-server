mod handler;
mod viewmodels;

use std::sync::Arc;
use library::SharedLibrary;
use player::SharedPlayer;
use provider::SharedProviders;
use router::Router;
use self::handler::*;

pub fn build(player: SharedPlayer, library: SharedLibrary, providers: SharedProviders) -> Router {
    router!(
        list_albums:    get  "/library/albums"     => ListAlbumsHandler::new(Arc::clone(&library)),
        get_album:      get  "/library/albums/:id" => GetAlbumHandler::new(Arc::clone(&library)),
        list_artists:   get  "/library/artists"    => ListArtistsHandler::new(Arc::clone(&library)),
        list_playlists: get  "/library/playlists"  => ListPlaylistsHandler::new(Arc::clone(&library), providers.clone()),
        list_tracks:    get  "/library/tracks"     => ListTracksHandler::new(Arc::clone(&library)),
        pause:          post "/player/pause"       => player::PausePlayerHandler::new(Arc::clone(&player)),
        play:           post "/player/play"        => player::PlayPlayerHandler::new(Arc::clone(&player)),
        next:           post "/player/next"        => player::NextPlayerHandler::new(Arc::clone(&player)),
        prev:           post "/player/prev"        => player::PrevPlayerHandler::new(Arc::clone(&player)),
        player_state:   get  "/player"             => player::PlayerStateHandler::new(Arc::clone(&player), Arc::clone(&library)),
        add_to_queue:   post "/queue/:id"          => queue::AddToQueueHandler::new(Arc::clone(&player), Arc::clone(&library)),
        get_queue:      get  "/queue"              => queue::GetQueueHandler::new(Arc::clone(&player), Arc::clone(&library))
    )
}