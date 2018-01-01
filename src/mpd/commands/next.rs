use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use library::GlobalLibrary;
use player::GlobalPlayer;

pub struct NextCommand {
}

impl NextCommand {
    pub fn new() -> NextCommand {
        NextCommand {}
    }
}

impl MpdCommand<()> for NextCommand {
    fn handle(&self, player: &GlobalPlayer, _library: &GlobalLibrary) -> Result<(), MpdError> {
        let mut player = player.lock().unwrap();
        player.next();
        Ok(())
    }
}