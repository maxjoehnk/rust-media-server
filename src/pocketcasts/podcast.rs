use pocketcasts::{PocketcastEpisode, PocketcastUser};
use library::{Album, Provider};
use reqwest::Client;
use reqwest::header;

const GET_EPISODES_URI: &'static str = "https://play.pocketcasts.com/web/episodes/find_by_podcast.json";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PocketcastPodcast {
    id: i32,
    pub uuid: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub url: Option<String>,
    pub thumbnail_url: Option<String>,
    #[serde(skip)]
    pub episodes: Vec<PocketcastEpisode>
}

impl PocketcastPodcast {
    pub fn get_episodes(&mut self, user: &PocketcastUser) -> Option<Vec<PocketcastEpisode>> {
        let body = json!({
            "uuid": self.uuid,
            "page": 1
        });
        let client = Client::new();
        let session = user.session.clone().expect("Login first");
        let mut cookies = header::Cookie::new();
        cookies.set("_social_session", session);
        let mut res = client.post(GET_EPISODES_URI)
            .header(cookies)
            .json(&body)
            .send()
            .unwrap();

        if !res.status().is_success() {
            return None;
        }

        let res: EpisodesResponse = res.json().unwrap();

        let episodes = res.result.episodes;

        self.episodes = episodes.clone();

        Some(episodes)
    }

    pub fn to_album(&self) -> Album {
        let tracks = self.episodes.clone().iter().map(|episode| episode.to_track()).collect();
        Album {
            title: self.title.clone(),
            artist: None,
            tracks,
            provider: Provider::Pocketcasts
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EpisodesResponse {
    status: String,
    token: String,
    copyright: String,
    result: EpisodesResponseResult
}

#[derive(Debug, Serialize, Deserialize)]
struct EpisodesResponseResult {
    episodes: Vec<PocketcastEpisode>
}