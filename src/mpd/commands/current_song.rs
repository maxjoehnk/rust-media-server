use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use library::GlobalLibrary;
use player::GlobalPlayer;
use mpd::song::MpdSong;

pub struct CurrentSongCommand {
}

impl CurrentSongCommand {
    pub fn new() -> CurrentSongCommand {
        CurrentSongCommand {}
    }
}

impl MpdCommand<Option<MpdSong>> for CurrentSongCommand {
    fn handle(&self, player: &GlobalPlayer, _library: &GlobalLibrary) -> Result<Option<MpdSong>, MpdError> {
        let player = player.lock().unwrap();
        let track = match player.queue.current() {
            Some(track) => Some(MpdSong::from(track.clone())),
            None => None
        };
        Ok(track)
    }
}