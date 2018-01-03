use provider::Provider;

#[derive(Debug, Clone, Serialize)]
pub struct Playlist {
    pub id: Option<usize>,
    pub title: String,
    pub tracks: Vec<String>,
    pub provider: Provider
}