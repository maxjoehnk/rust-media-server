use library::{Track, Album, Artist};

#[derive(Debug, Clone, Serialize)]
pub struct ProviderItem {
    pub label: String,
    pub track: Option<Track>,
    pub album: Option<Album>,
    pub artist: Option<Artist>
}

impl From<Track> for ProviderItem {
    fn from(track: Track) -> ProviderItem {
        ProviderItem {
            label: track.title.clone(),
            track: Some(track),
            album: None,
            artist: None
        }
    }
}

impl From<Album> for ProviderItem {
    fn from(album: Album) -> ProviderItem {
        ProviderItem {
            label: album.title.clone(),
            track: None,
            album: Some(album),
            artist: None
        }
    }
}

impl From<Artist> for ProviderItem {
    fn from(artist: Artist) -> ProviderItem {
        ProviderItem {
            label: artist.name.clone(),
            track: None,
            album: None,
            artist: Some(artist)
        }
    }
}