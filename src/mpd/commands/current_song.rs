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

impl MpdCommand<MpdSong> for CurrentSongCommand {
    fn handle(&self, player: &GlobalPlayer, library: &GlobalLibrary) -> Result<MpdSong, MpdError> {
        let player = player.lock().unwrap();
        let track = player.queue.current().unwrap().clone();
        Ok(MpdSong::from(track))
    }
}