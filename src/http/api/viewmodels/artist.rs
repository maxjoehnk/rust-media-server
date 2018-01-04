use library::{SharedLibrary, Album, Track, Artist};

#[derive(Clone, Debug, Serialize)]
pub struct ArtistModel {
    pub id: Option<usize>,
    pub name: String,
    pub albums: Vec<Album>,
    pub tracks: Vec<Track>
}

impl ArtistModel {
    pub fn from(artist: Artist, library: SharedLibrary) -> ArtistModel {
        let albums = library.albums.read().unwrap();
        let tracks = library.tracks.read().unwrap();
        let albums = albums
            .iter()
            .filter(|albums| albums.artist_id == artist.id)
            .cloned()
            .collect();
        let tracks = tracks
            .iter()
            .filter(|track| track.artist_id == artist.id)
            .cloned()
            .collect();
        ArtistModel {
            id: artist.id,
            name: artist.name,
            albums,
            tracks
        }
    }
}