mod episode;
mod podcast;
mod user;

use provider;
use library::{Track, GlobalLibrary, Album, Artist};
use rayon::prelude::*;

pub use self::podcast::PocketcastPodcast;
pub use self::episode::PocketcastEpisode;
pub use self::user::PocketcastUser;

#[derive(Debug, Clone, Deserialize)]
pub struct PocketcastsProvider {
    user: user::PocketcastUser
}

impl provider::ProviderInstance for PocketcastsProvider {
    fn title(&self) -> &'static str {
        "Pocketcasts"
    }

    fn sync(&mut self, library: GlobalLibrary) -> Result<usize, provider::SyncError> {
        let podcasts = self.user.get_subscriptions();
        let mut episodes: Vec<Track> = podcasts
            .par_iter()
            .cloned()
            .map(|mut podcast| {
                let episodes = podcast.get_episodes(&self.user).unwrap();
                (podcast, episodes)
            })
            .map(|(podcast, episodes)| {
                let mut artist = Artist::from(podcast.clone());
                let mut album = Album::from(podcast);
                library.add_artist(&mut artist);
                album.artist_id = artist.id.clone();
                library.add_album(&mut album);
                let tracks: Vec<Track> = episodes
                    .iter()
                    .cloned()
                    .map(Track::from)
                    .map(|mut track| {
                        track.album_id = album.id.clone();
                        track.artist_id = artist.id.clone();
                        track.coverart = album.coverart.clone();
                        track
                    })
                    .collect();
                tracks
            })
            .reduce(|| vec![], |mut a, b| {
                a.extend(b);
                a
            });
        let amount = episodes.len();
        library.add_tracks(&mut episodes);
        Ok(amount)
    }

    fn root(&self) -> provider::ProviderFolder {
        provider::ProviderFolder {
            folders: vec![
                "Subscriptions".to_owned(),
                "Top Charts".to_owned(),
                "Featured".to_owned(),
                "Trending".to_owned()
            ],
            items: vec![]
        }
    }

    fn navigate(&self, path: Vec<String>) -> Result<provider::ProviderFolder, provider::NavigationError> {
        let podcasts = match path[0].as_str() {
            "Subscriptions" => Ok(self.user.get_subscriptions()),
            "Top Charts" => Ok(self.user.get_top_charts()),
            "Featured" => Ok(self.user.get_featured()),
            "Trending" => Ok(self.user.get_trending()),
            _ => Err(provider::NavigationError::PathNotFound)
        };
        match path.len() {
            1 => podcasts.map(provider::ProviderFolder::from),
            2 => podcasts.and_then(|podcasts| {
                podcasts
                    .iter()
                    .find(|podcast| podcast.title == path[1])
                    .and_then(|podcast| podcast.get_episodes(&self.user))
                    .map(|episodes| {
                        episodes
                            .iter()
                            .cloned()
                            .map(|episode| Track::from(episode))
                            .map(provider::ProviderItem::from)
                            .collect()
                    })
                    .ok_or(provider::NavigationError::FetchError)
                    .map(|items| {
                        provider::ProviderFolder {
                            folders: vec![],
                            items
                        }
                    })
            }),
            _ => Err(provider::NavigationError::PathNotFound)
        }
    }

    fn search(&self, _query: String) -> Vec<provider::ProviderItem> {
        vec![]
    }
}

impl From<Vec<PocketcastPodcast>> for provider::ProviderFolder {
    fn from(podcasts: Vec<PocketcastPodcast>) -> provider::ProviderFolder {
        provider::ProviderFolder {
            folders: podcasts
                .iter()
                .cloned()
                .map(|podcast| podcast.title)
                .collect(),
            items: vec![]
        }
    }
}