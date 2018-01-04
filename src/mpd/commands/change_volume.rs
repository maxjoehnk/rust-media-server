use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use app::SharedApp;

pub struct ChangeVolumeCommand {
    pub volume: i32
}

impl ChangeVolumeCommand {
    pub fn new(volume: i32) -> ChangeVolumeCommand {
        ChangeVolumeCommand {
            volume
        }
    }
}

impl MpdCommand<()> for ChangeVolumeCommand {
    fn handle(&self, app: &SharedApp) -> Result<(), MpdError> {
        let mut player = app.player.lock().unwrap();
        let volume = (player.volume() as i32 + self.volume).min(100).max(0);
        player.set_volume(volume as u32);
        Ok(())
    }
}