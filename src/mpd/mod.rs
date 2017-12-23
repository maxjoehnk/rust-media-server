use slog;
use slog_term;
use std;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader, BufRead, Lines};
use std::thread;
use std::ops::Add;

use library::Library;
use player::Player;

use slog::Drain;
use std::sync::{Arc, Mutex};

//mod commands;
mod error;

lazy_static! {
    static ref logger: slog::Logger = slog::Logger::root(
        slog_term::FullFormat::new(slog_term::PlainSyncDecorator::new(std::io::stdout()))
            .build().fuse(), o!()
    );
}

#[derive(Deserialize, Clone)]
pub struct MpdConfig {
    ip: String,
    port: i32
}

pub fn open(config: MpdConfig, player: Player, library: Arc<Mutex<Library>>) {
    let listener = TcpListener::bind(format!("{}:{}", config.ip, config.port)).unwrap();
    info!(logger, "Listening on Port 6600");

    for stream in listener.incoming() {
        debug!(logger, "Connection opened");

        let mut player = player.clone();
        let mut library = library.clone();

        thread::spawn(move|| handle_client(stream.unwrap(), player, library));
    }
}

fn handle_client(mut stream: TcpStream, player: Player, library: Arc<Mutex<Library>>) {
    let mut reader = BufReader::new(stream);
    let header = "OK MPD 0.16.0\n";
    let result = reader.get_ref().write(header.as_bytes());
    match result {
        Ok(_) => trace!(logger, "< {:?}", &header),
        Err(e) => error!(logger, "{:?}", &e)
    }

    let mut in_list = false;

    loop {
        let line = reader.by_ref().lines().next();
        match line {
            Some(line) => {
                let line = line.unwrap();
                trace!(logger, "> {:?}", &line);
                let mut cmd: Option<MpdCommands> = None;
                if line == "command_list_ok_begin" {
                    let mut current = reader.by_ref().lines().next().expect("line").expect("line");
                    trace!(logger, "> {:?}", &current);
                    let mut cmds: Vec<MpdCommands> = vec![];
                    while current.as_str() != "command_list_end" {
                        match parse_single(current) {
                            Some(cmd) => cmds.push(cmd),
                            None => {}
                        }
                        current = reader.by_ref().lines().next().expect("line").expect("line");
                        trace!(logger, "> {:?}", &current);
                    }
                    cmd = Some(MpdCommands::CommandList(cmds));
                }else {
                    cmd = parse_single(line);
                }
                match cmd {
                    Some(cmd) => {
                        let mut result = handle_command(cmd, &player, &library);
                        result += "OK\n";
                        trace!(logger, "< {:?}", &result);
                        reader.get_ref().write(result.as_bytes());
                    },
                    None => {}
                }
            },
            None => {}
        }
        /*match line {
            Some(result) => {
                let input: String = result.unwrap();
                let result = match input.as_str() {
                    "command_list_ok_begin" => {
                        debug!(logger, "Command List");
                        in_list = true;
                        String::new()
                    },
                    "command_list_end" => {
                        debug!(logger, "Command List end");
                        in_list = false;
                        String::from("OK\n")
                    },
                    _ => {
                        let mut r = handle_command(input).unwrap();
                        if in_list {
                            r.push_str("list_OK\n");
                        }else {
                            r.push_str("OK\n");
                        }
                        r
                    }
                };
                trace!(logger, "< {:?}", &result);
                reader.get_ref().write(result.as_bytes());
            },
            None => break
        }*/
    }
}

#[derive(Debug)]
enum MpdCommands {
    Status,
    CurrentSong,
    CommandList(Vec<MpdCommands>),
    PlaylistChanges(String),
    Outputs,
    Decoders,
    Idle,
    NoIdle,
    ListPlaylists,
    ListPlaylist(String),
    ListPlaylistInfo(String),
    LoadPlaylist(String)
}

fn handle_status() -> String {
    let status = String::new();
    let status = status.add("volume: 100\n");
    let status = status.add("repeat: 0\n");
    let status = status.add("random: 0\n");
    let status = status.add("single: 0\n");
    let status = status.add("consume: 0\n");
    let status = status.add("playlist: 0\n");
    let status = status.add("playlistlength: 0\n");
    let status = status.add("xfade: 0\n");
    let status = status.add("state: stop\n");
    status
}

fn parse_single(line: String) -> Option<MpdCommands> {
    let reg = ::regex::Regex::new(r#"plchanges "(\d+)""#).unwrap();
    match line.as_str() {
        "status" => Some(MpdCommands::Status),
        "currentsong" => Some(MpdCommands::CurrentSong),
        "outputs" => Some(MpdCommands::Outputs),
        "decoders" => Some(MpdCommands::Decoders),
        "noidle" => Some(MpdCommands::NoIdle),
        "listplaylists" => Some(MpdCommands::ListPlaylists),
        _ => {
            if reg.is_match(line.as_str()) {
                let changes = reg.captures(line.as_str()).unwrap().get(0).unwrap().as_str().to_owned();
                return Some(MpdCommands::PlaylistChanges(changes));
            }
            None
        }
    }
}

fn handle_command(command: MpdCommands, player: &Player, library: &Arc<Mutex<Library>>) -> String {
    match command {
        MpdCommands::Status => handle_status(),
        MpdCommands::CurrentSong => String::new(),
        MpdCommands::CommandList(commands) => {
            let mut result = String::new();
            for command in commands {
                result += handle_command(command, player, library).as_str();
                result += "list_OK\n";
            }
            result
        },
        MpdCommands::PlaylistChanges(version) => {
            String::new()
        },
        MpdCommands::Outputs => {
            String::from("outputid: 0\noutputname: Default\noutputenabled: 0\n")
        },
        MpdCommands::Decoders => {
            String::new()
        },
        MpdCommands::Idle => {
            String::new()
        },
        MpdCommands::NoIdle => {
            String::new()
        },
        MpdCommands::ListPlaylists => {
            library.lock()
                .unwrap()
                .playlists
                .iter()
                .map(|playlist| format!("playlist: {}\nLast-Modified: {}\n", playlist.title, "2017-12-23T17:15:13Z").to_owned())
                .collect()
        },
        MpdCommands::ListPlaylist(name) => {
            let library = library.lock().unwrap();
            let playlist = library
                .playlists
                .iter()
                .find(|playlist| playlist.title == name);
            match playlist {
                Some(playlist) => {
                    println!("Found the playlist");
                    String::new()
                },
                None => String::new()
            }
        },
        MpdCommands::ListPlaylistInfo(name) => String::new(),
        MpdCommands::LoadPlaylist(name) => String::new()
    }
}