use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use app::SharedApp;

pub struct StopCommand {
}

impl StopCommand {
    pub fn new() -> StopCommand {
        StopCommand {}
    }
}

impl MpdCommand<()> for StopCommand {
    fn handle(&self, app: &SharedApp) -> Result<(), MpdError> {
        let mut player = app.player.lock().unwrap();
        player.stop();
        Ok(())
    }
}