mod commands;
mod error;
mod song;

use logger::logger;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader, BufRead};
use std::thread;

use library::GlobalLibrary;
use player::GlobalPlayer;
use provider::SharedProviders;

use serde_mpd;

use mpd::commands::MpdCommand;

#[derive(Deserialize, Clone)]
pub struct MpdConfig {
    pub ip: String,
    pub port: i32
}

pub fn open(config: MpdConfig, player: GlobalPlayer, library: GlobalLibrary, providers: SharedProviders) {
    let listener = TcpListener::bind(format!("{}:{}", config.ip, config.port)).unwrap();
    info!(logger, "[MPD] Listening on Port {}", config.port);

    for stream in listener.incoming() {
        debug!(logger, "[MPD] Connection opened");

        let player = player.clone();
        let library = library.clone();
        let providers = providers.clone();

        thread::spawn(move|| handle_client(stream.unwrap(), player, library, providers));
    }
}

fn handle_client(mut stream: TcpStream, player: GlobalPlayer, library: GlobalLibrary, providers: SharedProviders) {
    let mut reader = BufReader::new(stream);
    let header = "OK MPD 0.16.0\n";
    let result = reader.get_ref().write(header.as_bytes());
    match result {
        Ok(_) => trace!(logger, "< {:?}", &header),
        Err(e) => error!(logger, "[MPD] {:?}", &e)
    }

    loop {
        let line = reader.by_ref().lines().next();
        match line {
            Some(line) => {
                match line {
                    Ok(line) => {
                        trace!(logger, "> {:?}", &line);
                        let cmd: Result<MpdCommands, serde_mpd::Error> = if line == "command_list_ok_begin" {
                            let mut current = reader.by_ref().lines().next().expect("line").expect("line");
                            trace!(logger, "> {:?}", &current);
                            let mut cmds: Vec<MpdCommands> = vec![];
                            while current.as_str() != "command_list_end" {
                                match parse_single(current) {
                                    Ok(cmd) => cmds.push(cmd),
                                    Err(_) => {}
                                }
                                current = reader.by_ref().lines().next().expect("line").expect("line");
                                trace!(logger, "> {:?}", &current);
                            }
                            Ok(MpdCommands::CommandList(cmds))
                        }else {
                            parse_single(line)
                        };
                        match cmd {
                            Ok(MpdCommands::Idle) => {},
                            Ok(cmd) => {
                                let mut result = handle_mpd_command(cmd, &player, &library, &providers).unwrap();
                                result += "OK\n";
                                trace!(logger, "< {:?}", &result);
                                reader.get_ref().write(result.as_bytes());
                            },
                            Err(err) => {
                                error!(logger, "[MPD] {:?}", err);
                            }
                        }
                    },
                    Err(err) => {
                        error!(logger, "[MPD] {:?}", &err);
                        break;
                    }
                }
            },
            None => break
        }
    }

    debug!(logger, "[MPD] Connection closed");
}

#[derive(Debug, Deserialize)]
enum MpdCommands {
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "currentsong")]
    CurrentSong,
    #[serde(rename = "commandlist")]
    CommandList(Vec<MpdCommands>),
    #[serde(rename = "plchanges")]
    PlaylistChanges(String),
    #[serde(rename = "outputs")]
    Outputs,
    #[serde(rename = "decoders")]
    Decoders,
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "noidle")]
    NoIdle,
    #[serde(rename = "listplaylists")]
    ListPlaylists,
    #[serde(rename = "listplaylist")]
    ListPlaylist(String),
    #[serde(rename = "listplaylistinfo")]
    ListPlaylistInfo(String),
    #[serde(rename = "load")]
    LoadPlaylist(String),
    #[serde(rename = "lsinfo")]
    ListInfo(String),
    #[serde(rename = "next")]
    Next,
    #[serde(rename = "pause")]
    Pause(bool),
    #[serde(rename = "play")]
    Play(u64),
    #[serde(rename = "previous")]
    Previous,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "list")]
    List(String),
    #[serde(rename = "add")]
    Add(String),
    #[serde(rename = "addid")]
    AddId(String),
    #[serde(rename = "volume")]
    ChangeVolumeBy(i32),
    #[serde(rename = "setvol")]
    ChangeVolume(u32)
}

fn parse_single(line: String) -> Result<MpdCommands, serde_mpd::Error> {
    serde_mpd::from_str(line.as_str())
}

fn handle_mpd_command(cmd: MpdCommands, player: &GlobalPlayer, library: &GlobalLibrary, providers: &SharedProviders) -> Result<String, error::MpdError> {
    debug!(logger, "[MPD] Command: {:?}", &cmd);
    match cmd {
        MpdCommands::Status => commands::StatusCommand::new().handle(player, library, providers)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::CurrentSong => commands::CurrentSongCommand::new().handle(player, library, providers)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::Pause(true) => commands::PauseCommand::new().handle(player, library, providers)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::Stop => commands::StopCommand::new().handle(player, library, providers)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::ListInfo(path) => commands::ListInfoCommand::new(path).handle(player, library, providers)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::ListPlaylists => commands::ListPlaylistsCommand::new().handle(player, library, providers)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::ListPlaylist(name) => commands::ListPlaylistCommand::new(name).handle(player, library, providers)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::ListPlaylistInfo(name) => commands::ListPlaylistInfoCommand::new(name).handle(player, library, providers)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::LoadPlaylist(name) => commands::LoadPlaylistCommand::new(name).handle(player, library, providers)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::Previous => commands::PreviousCommand::new().handle(player, library, providers)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::Next => commands::NextCommand::new().handle(player, library, providers)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::Outputs => commands::OutputsCommand::new().handle(player, library, providers)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::List(ref t) if t == "Artist" => commands::ListArtistCommand::new().handle(player, library, providers)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::ChangeVolumeBy(volume) => commands::ChangeVolumeCommand::new(volume).handle(player, library, providers)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::ChangeVolume(volume) => commands::SetVolumeCommand::new(volume).handle(player, library, providers)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::CommandList(commands) => {
            let mut result = String::new();
            for command in commands {
                result += handle_mpd_command(command, player, library, providers).unwrap().as_str();
                result += "list_OK\n";
            }
            Ok(result)
        }
        _ => Ok(String::new())
    }
}