mod album;
mod artist;
mod library;
mod playlist;
mod track;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Provider {
    Pocketcasts,
    Soundcloud,
    GooglePlayMusic,
    Spotify,
    LocalMedia
}

pub use self::album::Album;
pub use self::artist::Artist;
pub use self::library::Library;
pub use self::playlist::Playlist;
pub use self::track::Track;