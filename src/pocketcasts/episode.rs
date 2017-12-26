use library::{Track, Provider};
use pocketcasts::PocketcastPodcast;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PocketcastEpisode<> {
    #[serde(skip)]
    //pub podcast: PocketcastPodcast,
    pub uuid: String,
    pub size: i32,
    pub title: String,
    pub url: String,
    //pub duration: String
}

impl PocketcastEpisode {
    pub fn to_track(&self) -> Track {
        Track {
            title: self.title.clone(),
            artist: None,
            album: None,//Some(self.podcast.to_album()),
            url: self.url.clone(),
            provider: Provider::Pocketcasts
        }
    }
}