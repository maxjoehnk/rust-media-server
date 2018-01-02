use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use library::GlobalLibrary;
use player::{GlobalPlayer, PlayerState};
use provider::SharedProviders;

#[derive(Debug, Serialize)]
pub struct AudioFormat {
    samplerate: i32,
    bits: i32,
    channels: i32
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    volume: i32,
    repeat: bool,
    random: bool,
    single: bool,
    consume: bool,
    playlist: u32,
    playlistlength: i32,
    state: PlayerState,
//    song: i32,
//    songid: i32,
//    nextsong: i32,
//    nextsongid: i32,
//    time: i32,
//    elapsed: i32,
//    duration: i32,
//    bitrate: i32,
    xfade: i32,
//    mixrampdb: i32,
//    mixrampdelay: i32,
//    audio: AudioFormat,
//    updating_db: i32,
//    error: String
}

pub struct StatusCommand {
}

impl StatusCommand {
    pub fn new() -> StatusCommand {
        StatusCommand {}
    }
}

impl MpdCommand<StatusResponse> for StatusCommand {
    fn handle(&self, player: &GlobalPlayer, _library: &GlobalLibrary, _providers: &SharedProviders) -> Result<StatusResponse, MpdError> {
        let player = player.lock().unwrap();
        Ok(StatusResponse {
            volume: 0,
            repeat: false,
            random: false,
            single: false,
            consume: false,
            playlist: 0,
            playlistlength: 0,
            state: player.state.clone(),
            xfade: 0
        })
    }
}