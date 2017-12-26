use library::Track;

#[derive(Debug, Clone, Serialize)]
pub struct Playlist {
    pub title: String,
    pub tracks: Vec<Track>
}