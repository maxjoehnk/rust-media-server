use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use mpd::song::MpdSong;
use app::SharedApp;

// TODO: parse MpdCommands enum

pub struct CommandsCommand {
}

#[derive(Serialize, Debug)]
pub struct Command {
    command: String
}

impl Command {
    fn new(label: &'static str) -> Command {
        Command {
            command: label.to_owned()
        }
    }
}

impl CommandsCommand {
    pub fn new() -> CommandsCommand {
        CommandsCommand {}
    }
}

impl MpdCommand<Vec<Command>> for CommandsCommand {
    fn handle(&self, app: &SharedApp) -> Result<Vec<Command>, MpdError> {
        Ok(vec![
            Command::new("status"),
            Command::new("currentsong"),
            Command::new("commandlist"),
            Command::new("plchanges"),
            Command::new("outputs"),
            Command::new("decoders"),
            Command::new("idle"),
            Command::new("noidle"),
            Command::new("listplaylists"),
            Command::new("listplaylist"),
            Command::new("listplaylistinfo"),
            Command::new("load"),
            Command::new("lsinfo"),
            Command::new("next"),
            Command::new("pause"),
            Command::new("play"),
            Command::new("previous"),
            Command::new("stop"),
            Command::new("list"),
            Command::new("add"),
            Command::new("addid"),
            Command::new("volume"),
            Command::new("setvol"),
            Command::new("commands"),
            Command::new("tagtypes"),
        ])
    }
}