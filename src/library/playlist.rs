use library::Track;

#[derive(Debug, Clone)]
pub struct Playlist {
    pub title: String,
    pub tracks: Vec<Track>
}