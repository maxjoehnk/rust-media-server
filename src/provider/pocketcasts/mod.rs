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
            label: "Pocketcasts".to_owned(),
            folders: vec![
                provider::ProviderFolder {
                    label: "Subscriptions".to_owned(),
                    folders: vec![],
                    items: self.user
                        .get_subscriptions()
                        .iter()
                        .cloned()
                        .map(Album::from)
                        .map(provider::ProviderItem::from)
                        .collect()
                }
            ],
            items: vec![]
        }
    }

    fn search(&self, _query: String) -> Vec<provider::ProviderItem> {
        vec![]
    }
}