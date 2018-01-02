use library::{GlobalLibrary, Album, Track, Artist};
use provider::Provider;

#[derive(Clone, Debug, Serialize)]
pub struct AlbumModel {
    pub id: Option<usize>,
    pub title: String,
    pub artist: Option<Artist>,
    pub tracks: Vec<Track>,
    pub provider: Provider
}

impl AlbumModel {
    pub fn from(album: Album, library: GlobalLibrary) -> AlbumModel {
        let tracks = library.tracks.read().unwrap();
        let artists = library.artists.read().unwrap();
        let tracks = tracks
            .iter()
            .filter(|track| track.album_id == album.id)
            .cloned()
            .collect();
        let artist = artists
            .iter()
            .cloned()
            .find(|artist| artist.id == album.artist_id);
        AlbumModel {
            id: album.id,
            title: album.title,
            artist,
            tracks,
            provider: album.provider
        }
    }
}