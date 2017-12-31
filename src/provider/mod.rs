pub mod soundcloud;
pub mod pocketcasts;
mod sync_error;
mod item;
mod folder;

pub use self::item::ProviderItem;
pub use self::folder::ProviderFolder;
pub use self::sync_error::SyncError;

use library::GlobalLibrary;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Provider {
    Pocketcasts,
    Soundcloud,
    GooglePlayMusic,
    Spotify,
    LocalMedia
}

pub trait ProviderInstance {
    fn sync(&mut self, library: GlobalLibrary) -> Result<(), SyncError>;
    fn root(&self) -> ProviderFolder;
    fn search(&self, query: String) -> Vec<ProviderItem>;
}
