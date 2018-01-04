use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use mpd::song::MpdSong;
use app::SharedApp;

pub struct CurrentSongCommand {
}

impl CurrentSongCommand {
    pub fn new() -> CurrentSongCommand {
        CurrentSongCommand {}
    }
}

impl MpdCommand<Option<MpdSong>> for CurrentSongCommand {
    fn handle(&self, app: &SharedApp) -> Result<Option<MpdSong>, MpdError> {
        let player = app.player.lock().unwrap();
        let track = match player.queue.current() {
            Some(track) => Some(MpdSong::from(track.clone())),
            None => None
        };
        Ok(track)
    }
}