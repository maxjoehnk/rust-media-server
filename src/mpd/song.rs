use library::Track;

#[derive(Debug, Serialize)]
pub struct MpdSong {
    file: String,
    #[serde(rename = "Title")]
    title: Option<String>,
    #[serde(rename = "Id")]
    id: i64,
    #[serde(rename = "Track")]
    track: i64
}

impl From<Track> for MpdSong {
    fn from(track: Track) -> MpdSong {
        MpdSong {
            file: track.url,
            title: Some(track.title),
            id: 1,
            track: 0
        }
    }
}