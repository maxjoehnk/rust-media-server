mod track;

use soundcloud;
use provider;
use library::GlobalLibrary;

#[derive(Debug, Clone, Deserialize)]
pub struct SoundcloudProvider {
    client_id: String
}

impl provider::ProviderInstance for SoundcloudProvider {
    fn sync(&mut self, _library: GlobalLibrary) -> Result<(), provider::SyncError> {
        Ok(())
    }
    fn root(&self) -> provider::ProviderFolder {
        provider::ProviderFolder::empty("Soundcloud".to_owned())
    }
    fn search(&self, query: String) -> Vec<provider::ProviderItem> {
        let client = soundcloud::Client::new(self.client_id.as_str());
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
}