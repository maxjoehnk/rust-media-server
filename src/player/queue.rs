use library::{Track, Playlist};

#[derive(Debug, Clone)]
pub struct Queue {
    tracks: Vec<Track>,
    current: Option<usize>
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            tracks: vec![],
            current: Some(0)
        }
    }

    pub fn add_playlist(&mut self, playlist: Playlist) {
        self.tracks.append(&mut playlist.tracks.to_vec());
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    pub fn add_multiple(&mut self, tracks: Vec<Track>) {
        self.tracks.append(&mut tracks.to_vec());
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
    }

    pub fn next(&mut self) -> Option<&Track> {
        self.current
            .and_then(move|index| {
                let next_index = index + 1;
                if next_index >= self.tracks.len() {
                    return None;
                }
                self.current = Some(next_index);
                self.tracks.get(next_index)
            })
    }

    pub fn current(&self) -> Option<&Track> {
        self.current
            .and_then(move|index| self.tracks.get(index))
    }
}