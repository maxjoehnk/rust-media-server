use provider::Provider;

#[derive(Clone, Debug, Serialize)]
pub struct Track {
    pub id: Option<usize>,
    pub title: String,
    pub artist_id: Option<usize>,
    pub album_id: Option<usize>,
    pub stream_url: String,
    pub provider: Provider,
    pub path: String
}