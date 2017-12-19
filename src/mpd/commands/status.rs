use mpd::error::MpdError;
use mpd::commands::{MpdRequest, MpdResponse};

#[derive(Debug)]
pub struct StatusRequest {}

#[derive(Debug)]
pub enum PlayingState {
    Play,
    Stop,
    Pause
}

#[derive(Debug)]
pub struct AudioFormat {
    samplerate: i32,
    bits: i32,
    channels: i32
}

#[derive(Debug)]
pub struct StatusResponse {
    volume: i32,
    repeat: bool,
    random: bool,
    single: bool,
    consume: bool,
    playlist: u32,
    playlistlength: i32,
    state: PlayingState,
    song: i32,
    songid: i32,
    nextsong: i32,
    nextsongid: i32,
    time: i32,
    elapsed: i32,
    duration: i32,
    bitrate: i32,
    xfade: i32,
    mixrampdb: i32,
    mixrampdelay: i32,
    audio: AudioFormat,
    updating_db: i32,
    error: String
}

impl MpdRequest for StatusRequest {
    fn parse(command: String) -> Option<StatusRequest> {
        match command.as_str() {
            "status" => Some(StatusRequest {}),
            _ => None
        }
    }

    fn handle(&self) -> Result<StatusResponse, MpdError> {
        Ok(StatusResponse {
            volume: 0,
            repeat: false,
            random: false,
            single: false,
            consume: false,
            playlist: 0,
            playlistlength: 0,
            state: PlayingState::Stop,
            song: 0,
            songid: 0,
            nextsong: 0,
            nextsongid: 0,
            time: 0,
            elapsed: 0,
            duration: 0,
            bitrate: 0,
            xfade: 0,
            mixrampdb: 0,
            mixrampdelay: 0,
            audio: AudioFormat {
                bits: 0,
                samplerate: 0,
                channels: 0
            },
            updating_db: 0,
            error: String::new()
        })
    }
}

impl MpdResponse for StatusResponse {
    fn serialize(&self) -> String {
        String::new()
    }
}

