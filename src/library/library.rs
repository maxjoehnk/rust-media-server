use library::{Artist, Album, Track, Playlist};

#[derive(Debug, Clone, Serialize)]
pub struct Library {
    pub albums: Vec<Album>,
    pub artists: Vec<Artist>,
    pub tracks: Vec<Track>,
    pub playlists: Vec<Playlist>
}

impl Library {
    pub fn new() -> Library {
        Library {
            albums: vec![],
            artists: vec![],
            tracks: vec![],
            playlists: vec![]
        }
    }

    pub fn add_all(&mut self, tracks: &mut Vec<Track>) {
        self.tracks.append(tracks);
    }

    pub fn search(&self, query: &'static str) -> Vec<Track> {
        self.tracks
            .clone()
            .into_iter()
            .filter(|track| track.title.contains(query))
            .collect()
    }
}