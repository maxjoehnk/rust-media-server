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
        PlaylistModel {
            id: playlist.id,
            title: playlist.title,
            tracks: playlist.tracks,
            provider: playlist.provider
        }
    }
}