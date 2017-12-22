use pocketcasts::{PocketcastEpisode, PocketcastUser};
use library::Album;
use reqwest::Client;
use reqwest::header;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PocketcastPodcast {
    id: i32,
    pub uuid: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub url: Option<String>,
    pub thumbnail_url: Option<String>
}

impl PocketcastPodcast {
    pub fn get_episodes(&self, user: &PocketcastUser) -> Option<Vec<PocketcastEpisode>> {
        let uri = "https://play.pocketcasts.com/web/episodes/find_by_podcast.json";
        let body = json!({
            "uuid": self.uuid,
            "page": 1
        });
        let client = Client::new();
        let session = user.session.clone().expect("Login first");
        let mut cookies = header::Cookie::new();
        cookies.set("_social_session", session);
        let mut res = client.post(uri)
            .header(cookies)
            .json(&body)
            .send()
            .unwrap();

        if !res.status().is_success() {
            return None;
        }

        let res: EpisodesResponse = res.json().unwrap();

        Some(res.result.episodes)
    }

    pub fn to_album(&self) -> Album {
        let tracks = vec![]; //self.episodes.clone().iter().map(|episode| episode.to_track()).collect();
        Album {
            title: self.title.clone(),
            artist: None,
            tracks
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