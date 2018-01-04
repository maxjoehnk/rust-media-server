use library::{SharedLibrary, Playlist, Track};
use provider::{Provider, SharedProviders};
use rayon::prelude::*;

#[derive(Clone, Debug, Serialize)]
pub struct PlaylistModel {
    pub id: Option<usize>,
    pub title: String,
    pub tracks: Vec<Track>,
    pub provider: Provider
}

impl PlaylistModel {
    pub fn from(playlist: Playlist, library: SharedLibrary, providers: SharedProviders) -> PlaylistModel {
        let tracks = playlist
            .tracks
            .par_iter()
            .map(|uri| library.resolve_track(providers.clone(), &uri))
            .filter(|track| track.is_some())
            .map(|track| track.unwrap())
            .collect();
        PlaylistModel {
            id: playlist.id,
            title: playlist.title,
            tracks,
            provider: playlist.provider
        }
    }
}