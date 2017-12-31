use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use library::GlobalLibrary;
use player::GlobalPlayer;

pub struct PauseCommand {
}

impl PauseCommand {
    pub fn new() -> PauseCommand {
        PauseCommand {}
    }
}

impl MpdCommand<()> for PauseCommand {
    fn handle(&self, player: &GlobalPlayer, library: &GlobalLibrary) -> Result<(), MpdError> {
        let mut player = player.lock().unwrap();
        player.pause();
        Ok(())
    }
}