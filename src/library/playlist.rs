use library::Track;
use provider::Provider;

#[derive(Debug, Clone, Serialize)]
pub struct Playlist {
    pub title: String,
    pub tracks: Vec<Track>,
    pub provider: Provider
}