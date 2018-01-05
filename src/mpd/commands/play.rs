use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use app::SharedApp;

pub struct PlayCommand {
}

impl PlayCommand {
    pub fn new() -> PlayCommand {
        PlayCommand {}
    }
}

impl MpdCommand<()> for PlayCommand {
    fn handle(&self, app: &SharedApp) -> Result<(), MpdError> {
        let mut player = app.player.lock().unwrap();
        player.play();
        Ok(())
    }
}