use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use library::GlobalLibrary;
use player::GlobalPlayer;
use provider::SharedProviders;

pub struct PreviousCommand {
}

impl PreviousCommand {
    pub fn new() -> PreviousCommand {
        PreviousCommand {}
    }
}

impl MpdCommand<()> for PreviousCommand {
    fn handle(&self, player: &GlobalPlayer, _library: &GlobalLibrary, _providers: &SharedProviders) -> Result<(), MpdError> {
        let mut player = player.lock().unwrap();
        player.prev();
        Ok(())
    }
}