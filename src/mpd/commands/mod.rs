use std;
use mpd::error::MpdError;

mod status;
mod commands;

pub trait MpdRequest: std::marker::Sized {
    fn parse(command: String) -> Option<Self>;
    fn handle(&self) -> Result<MpdResponse, MpdError>;
}

pub trait MpdResponse {
    fn serialize(&self) -> String;
}

fn unwrap(result: Result<MpdResponse, MpdError>) -> String {
    match result {
        Ok(res) => res.serialize(),
        Err(err) => err.message
    }
}

pub fn parse(command: String) -> String {
    let res = status::StatusRequest::parse(command)
        .or_else(|| commands::CommandsRequest::parse(command))
        .and_then(|req: MpdRequest| req.handle());
    unwrap(res)
}