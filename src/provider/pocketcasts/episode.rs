use library::Track;
use provider::Provider;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PocketcastEpisode<> {
    pub uuid: String,
    pub size: i32,
    pub title: String,
    pub url: String,
    //pub duration: String
}

impl From<PocketcastEpisode> for Track {
    fn from(episode: PocketcastEpisode) -> Track {
        Track {
            id: None,
            title: episode.title,
            artist_id: None,
            album_id: None,
            stream_url: episode.url,
            provider: Provider::Pocketcasts,
            path: format!("pocketcasts:{}", episode.uuid),
            coverart: None
        }
    }
}