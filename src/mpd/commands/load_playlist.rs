use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use library::GlobalLibrary;
use player::GlobalPlayer;

pub struct LoadPlaylistCommand {
    name: String
}

impl LoadPlaylistCommand {
    pub fn new(name: String) -> LoadPlaylistCommand {
        LoadPlaylistCommand {
            name
        }
    }
}

impl MpdCommand<()> for LoadPlaylistCommand {
    fn handle(&self, player: &GlobalPlayer, library: &GlobalLibrary) -> Result<(), MpdError> {
        let playlist = library
            .lock()
            .unwrap()
            .playlists
            .iter()
            .find(|playlist| playlist.title == self.name)
            .unwrap()
            .clone();
        let mut player = player.lock().unwrap();
        player.queue.add_playlist(playlist);
        Ok(())
    }
}