use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use app::SharedApp;

pub struct PauseCommand {
}

impl PauseCommand {
    pub fn new() -> PauseCommand {
        PauseCommand {}
    }
}

impl MpdCommand<()> for PauseCommand {
    fn handle(&self, app: &SharedApp) -> Result<(), MpdError> {
        let mut player = app.player.lock().unwrap();
        player.pause();
        Ok(())
    }
}