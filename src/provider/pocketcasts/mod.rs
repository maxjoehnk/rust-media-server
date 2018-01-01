mod episode;
mod podcast;
mod user;

use provider;
use library::{Track, GlobalLibrary};
use rayon::prelude::*;

pub use self::podcast::PocketcastPodcast;
pub use self::episode::PocketcastEpisode;
pub use self::user::PocketcastUser;

#[derive(Debug, Clone, Deserialize)]
pub struct PocketcastsProvider {
    user: user::PocketcastUser
}

impl provider::ProviderInstance for PocketcastsProvider {
    fn sync(&mut self, library: GlobalLibrary) -> Result<(), provider::SyncError> {
        let mut podcasts = self.user.get_subscriptions();
        let mut episodes: Vec<Track> = podcasts
            .par_iter_mut()
            .map(|podcast| {
                podcast.get_episodes(&self.user).unwrap()
            })
            .reduce(|| vec![], |mut a, mut b| {
                a.append(&mut b);
                a
            })
            .iter()
            .map(|episode| episode.to_track())
            .collect();
        let mut library = library.lock()?;
        library.add_tracks(&mut episodes);
        let mut albums = podcasts.par_iter()
            .map(|podcast| podcast.to_album())
            .collect();
        library.add_albums(&mut albums);
        Ok(())
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
                        .map(|podcast| podcast.to_album().into())
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