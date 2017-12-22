use library::{Track, Album};

#[derive(Clone, Debug)]
pub struct Artist {
    pub name: String,
    pub tracks: Vec<Track>,
    pub albums: Vec<Album>
}