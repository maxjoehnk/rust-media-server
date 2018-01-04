use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use app::SharedApp;

pub struct SetVolumeCommand {
    pub volume: u32
}

impl SetVolumeCommand {
    pub fn new(volume: u32) -> SetVolumeCommand {
        SetVolumeCommand {
            volume
        }
    }
}

impl MpdCommand<()> for SetVolumeCommand {
    fn handle(&self, app: &SharedApp) -> Result<(), MpdError> {
        let mut player = app.player.lock().unwrap();
        player.set_volume(self.volume);
        Ok(())
    }
}