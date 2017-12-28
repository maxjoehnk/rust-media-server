use library::{Album, Artist, Provider};

#[derive(Clone, Debug, Serialize)]
pub struct Track {
    pub title: String,
    pub artist: Option<Artist>,
    pub album: Option<Album>,
    pub url: String,
    pub provider: Provider
}