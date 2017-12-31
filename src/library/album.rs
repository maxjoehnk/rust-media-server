use library::{Artist, Track};
use provider::Provider;

#[derive(Clone, Debug, Serialize)]
pub struct Album {
    pub title: String,
    pub artist: Option<Artist>,
    pub tracks: Vec<Track>,
    pub provider: Provider
}