use slog;
use slog_term;
use std;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader, BufRead};
use std::thread;
use std::ops::Add;

use slog::Drain;

lazy_static! {
    static ref logger: slog::Logger = slog::Logger::root(
        slog_term::FullFormat::new(slog_term::PlainSyncDecorator::new(std::io::stdout()))
            .build().fuse(), o!()
    );
}

#[derive(Deserialize)]
struct MpdConfig {
    ip: String,
    port: i32
}

mod commands;
mod error;

fn handle_status() -> Option<String> {
    let status = String::new();
    let status = status.add("volume 100\n");
    let status = status.add("repeat 0\n");
    let status = status.add("random 0\n");
    let status = status.add("single 0\n");
    let status = status.add("consume 0\n");
    let status = status.add("playlist 0\n");
    let status = status.add("playlistlength 0\n");
    let status = status.add("state stop\n");
    let status = status.add("song 0\n");
    let status = status.add("songid 0\n");
    let status = status.add("nextsong 0\n");
    let status = status.add("nextsongid 0\n");
    let status = status.add("time 0\n");
    let status = status.add("elapsed 0\n");
    let status = status.add("bitrate 0\n");
    let status = status.add("xfade 0\n");
    let status = status.add("mixrampdb 0\n");
    let status = status.add("mixrampdelay 0\n");
    let status = status.add("audio \"48000:24000:2\"\n");
    let status = status.add("updating_db 0\n");
    let status = status.add("error\n");
    let status = status.add("OK\n");

    Some(status)
}

fn handle_command(line: String) -> Option<String> {
    trace!(logger, "> {:?}", &line);
    match line.as_str() {
        "status" => {
            debug!(logger, "Status requested");
            handle_status()
        },
        _ => {
            debug!(logger, "Unknown Command: {:?}", &line);
            None
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    let header = "OK MPD 0.16.0\n";
    let result = reader.get_ref().write(header.as_bytes());
    match result {
        Ok(_) => trace!(logger, "< {:?}", &header),
        Err(e) => error!(logger, "{:?}", &e)
    }

    loop {
        let line = reader.by_ref().lines().next();
        match line {
            Some(result) => {
                let input: String = result.unwrap();
                let result = match input.as_str() {
                    "command_list_ok_begin" => {
                        debug!(logger, "Command List");
                        String::new()
                    },
                    _ => handle_command(input).unwrap()
                };
                trace!(logger, "< {:?}", &result);
                reader.get_ref().write(result.as_bytes());
            },
            None => break
        }
    }
}

pub extern fn open(descriptor: &'static str) {
    thread::Builder::new().spawn(|| {
        let listener = TcpListener::bind(descriptor).unwrap();
        info!(logger, "Listening on Port 6600");

        for stream in listener.incoming() {
            debug!(logger, "Connection opened");

            thread::Builder::new().spawn(move|| handle_client(stream.unwrap()));
        }
    })
}