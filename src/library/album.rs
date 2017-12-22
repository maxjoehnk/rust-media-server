use library::{Artist, Track};

#[derive(Clone, Debug)]
pub struct Album {
    pub title: String,
    pub artist: Option<Artist>,
    pub tracks: Vec<Track>
}