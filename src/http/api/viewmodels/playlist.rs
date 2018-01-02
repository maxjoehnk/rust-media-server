use library::{GlobalLibrary, Playlist, Track};
use provider::Provider;

#[derive(Clone, Debug, Serialize)]
pub struct PlaylistModel {
    pub id: Option<usize>,
    pub title: String,
    pub tracks: Vec<Track>,
    pub provider: Provider
}

impl PlaylistModel {
    pub fn from(playlist: Playlist, library: GlobalLibrary) -> PlaylistModel {
        let tracks = playlist
            .tracks
            .iter()
            .map(|id| library.get_track(&id))
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