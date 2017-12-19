use mpd::error::MpdError;
use mpd::commands::{MpdRequest, MpdResponse};

#[derive(Debug)]
pub struct CommandsRequest {}

#[derive(Debug)]
pub struct CommandsResponse {
}

impl MpdRequest for CommandsRequest {
    fn parse(command: String) -> Option<CommandsRequest> {
        match command.as_str() {
            "commands" => Some(CommandsRequest {}),
            _ => None
        }
    }

    fn handle(&self) -> Result<CommandsResponse, MpdError> {
        Ok(CommandsResponse {})
    }
}

impl MpdResponse for CommandsResponse {
    fn serialize(&self) -> String {
        String::new()
    }
}