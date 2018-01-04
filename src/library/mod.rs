mod album;
mod artist;
mod library;
mod playlist;
mod track;

use std::sync::Arc;

pub use self::album::Album;
pub use self::artist::Artist;
pub use self::library::Library;
pub use self::playlist::Playlist;
pub use self::track::Track;

pub type SharedLibrary = Arc<Library>;