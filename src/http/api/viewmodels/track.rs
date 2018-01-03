use library::{GlobalLibrary, Album, Track, Artist};
use provider::Provider;

#[derive(Clone, Debug, Serialize)]
pub struct TrackModel {
    pub id: Option<usize>,
    pub title: String,
    pub artist: Option<Artist>,
    pub album: Option<Album>,
    pub stream_url: String,
    pub path: String,
    pub provider: Provider,
    pub coverart: Option<String>,
    pub duration: Option<u64>
}

impl TrackModel {
    pub fn from(track: Track, library: GlobalLibrary) -> TrackModel {
        let artist = track.artist_id
            .and_then(|id| library.get_artist(&id));
        let album = track.album_id
            .and_then(|id| library.get_album(&id));
        TrackModel {
            id: track.id,
            title: track.title,
            stream_url: track.stream_url,
            path: track.path,
            provider: track.provider,
            coverart: track.coverart,
            duration: track.duration,
            artist,
            album
        }
    }
}