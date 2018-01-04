use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use app::SharedApp;

pub struct PreviousCommand {
}

impl PreviousCommand {
    pub fn new() -> PreviousCommand {
        PreviousCommand {}
    }
}

impl MpdCommand<()> for PreviousCommand {
    fn handle(&self, app: &SharedApp) -> Result<(), MpdError> {
        let mut player = app.player.lock().unwrap();
        player.prev();
        Ok(())
    }
}