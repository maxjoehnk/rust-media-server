use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use app::SharedApp;

pub struct NextCommand {
}

impl NextCommand {
    pub fn new() -> NextCommand {
        NextCommand {}
    }
}

impl MpdCommand<()> for NextCommand {
    fn handle(&self, app: &SharedApp) -> Result<(), MpdError> {
        let mut player = app.player.lock().unwrap();
        player.next();
        Ok(())
    }
}