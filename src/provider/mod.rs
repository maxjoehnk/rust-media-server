pub mod soundcloud;
pub mod pocketcasts;
mod explorer;
mod sync_error;
mod item;
mod folder;

pub use self::item::ProviderItem;
pub use self::folder::ProviderFolder;
pub use self::sync_error::SyncError;
pub use self::explorer::Explorer;

use std::sync::{Arc, Mutex};
use library::GlobalLibrary;

pub type SharedProviders = Vec<Arc<Mutex<Box<ProviderInstance + Send>>>>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Provider {
    Pocketcasts,
    Soundcloud,
    GooglePlayMusic,
    Spotify,
    LocalMedia
}

pub trait ProviderInstance {
    fn title(&self) -> &'static str;
    fn sync(&mut self, library: GlobalLibrary) -> Result<usize, SyncError>;
    fn root(&self) -> ProviderFolder;
    fn search(&self, query: String) -> Vec<ProviderItem>;
}