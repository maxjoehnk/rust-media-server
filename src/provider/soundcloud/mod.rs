mod track;
mod playlist;

use soundcloud;
use provider;
use library::{GlobalLibrary, Playlist, Track};
use url::Url;
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize)]
pub struct SoundcloudProvider {
    client_id: String,
    auth_token: Option<String>
}

impl SoundcloudProvider {
    fn client(&self) -> soundcloud::Client {
        let mut client = soundcloud::Client::new(self.client_id.as_str());
        if self.auth_token.is_some() {
            let token = self.auth_token.clone().unwrap();
            client.authenticate_with_token(token);
        }
        client
    }
}

impl provider::ProviderInstance for SoundcloudProvider {
    fn title(&self) -> &'static str {
        "Soundcloud"
    }

    fn uri_scheme(&self) -> &'static str { "soundcloud" }

    fn sync(&mut self, library: GlobalLibrary) -> Result<provider::SyncResult, provider::SyncError> {
        let client = self.client();
        let mut playlists: Vec<Playlist> = client
            .playlists()?
            .iter()
            .cloned()
            .map(playlist::SoundcloudPlaylist::from)
            .map(Playlist::from)
            .collect();
        library.add_playlists(&mut playlists);
        Ok(provider::SyncResult {
            tracks: 0,
            albums: 0,
            artists: 0,
            playlists: playlists.len()
        })
    }
    fn root(&self) -> provider::ProviderFolder {
        provider::ProviderFolder::empty()
    }
    fn navigate(&self, path: Vec<String>) -> Result<provider::ProviderFolder, provider::NavigationError> {
        Err(provider::NavigationError::PathNotFound)
    }
    fn search(&self, query: String) -> Vec<provider::ProviderItem> {
        let client = self.client();
        client.tracks()
            .query(Some(query))
            .get()
            .unwrap()
            .unwrap_or(vec![])
            .iter()
            .filter(|track| track.stream_url.is_some())
            .cloned()
            .map(|track| track.into())
            .collect()
    }
    fn resolve_track(&self, uri: &String) -> Option<Track> {
        let id = &uri["soundcloud://".len()..];
        usize::from_str(id).ok()
            .and_then(|id| {
                let client = self.client();
                client.tracks().id(id).get().ok()
            })
            .map(|mut track| {
                if track.stream_url.is_some() {
                    track.stream_url = Some(format!("{}?client_id={}", track.stream_url.unwrap(), self.client_id.clone()))
                }
                track
            })
            .map(track::SoundcloudTrack::from)
            .map(Track::from)
    }
}

impl From<soundcloud::Error> for provider::sync_error::SyncError {
    fn from(error: soundcloud::Error) -> provider::sync_error::SyncError {
        provider::sync_error::SyncError::ConfigurationError
    }
}