use library::Track;

#[derive(Debug, Serialize)]
pub struct MpdSong {
    file: String,
    #[serde(rename = "Title")]
    title: Option<String>,
    #[serde(rename = "Id")]
    id: usize,
    #[serde(rename = "Track")]
    track: usize
}

impl From<Track> for MpdSong {
    fn from(track: Track) -> MpdSong {
        MpdSong {
            file: track.path,
            title: Some(track.title),
            id: track.id.unwrap_or(0),
            track: 0
        }
    }
}