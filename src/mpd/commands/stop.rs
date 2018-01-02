use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use library::GlobalLibrary;
use player::GlobalPlayer;
use provider::SharedProviders;

pub struct StopCommand {
}

impl StopCommand {
    pub fn new() -> StopCommand {
        StopCommand {}
    }
}

impl MpdCommand<()> for StopCommand {
    fn handle(&self, player: &GlobalPlayer, _library: &GlobalLibrary, _providers: &SharedProviders) -> Result<(), MpdError> {
        let mut player = player.lock().unwrap();
        player.stop();
        Ok(())
    }
}