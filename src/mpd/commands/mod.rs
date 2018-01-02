use mpd::error::MpdError;
use library::GlobalLibrary;
use player::GlobalPlayer;
use provider::SharedProviders;

mod current_song;
mod list_artist;
mod list_info;
mod list_playlist;
mod list_playlist_info;
mod list_playlists;
mod load_playlist;
mod next;
mod outputs;
mod pause;
mod previous;
mod status;
mod stop;

pub use self::current_song::CurrentSongCommand;
pub use self::list_artist::ListArtistCommand;
pub use self::list_info::ListInfoCommand;
pub use self::list_playlist::ListPlaylistCommand;
pub use self::list_playlist_info::ListPlaylistInfoCommand;
pub use self::list_playlists::ListPlaylistsCommand;
pub use self::load_playlist::LoadPlaylistCommand;
pub use self::next::NextCommand;
pub use self::outputs::OutputsCommand;
pub use self::pause::PauseCommand;
pub use self::previous::PreviousCommand;
pub use self::status::StatusCommand;
pub use self::stop::StopCommand;

pub trait MpdCommand<T> {
    fn handle(&self, player: &GlobalPlayer, library: &GlobalLibrary, providers: &SharedProviders) -> Result<T, MpdError>;
}