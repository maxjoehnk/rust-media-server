use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use app::SharedApp;

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
    fn handle(&self, app: &SharedApp) -> Result<(), MpdError> {
        let tracks = app
            .library
            .playlists
            .read()
            .unwrap()
            .iter()
            .find(|playlist| playlist.title == self.name)
            .cloned()
            .unwrap()
            .tracks;
        let mut player = app.player.lock().unwrap();
        player.queue.add_multiple(tracks);
        Ok(())
    }
}