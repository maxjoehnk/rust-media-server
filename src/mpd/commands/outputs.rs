use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use app::SharedApp;

#[derive(Debug, Serialize)]
pub struct OutputEntry {
    #[serde(rename = "outputid")]
    id: i64,
    #[serde(rename = "outputname")]
    name: String,
    #[serde(rename = "outputenabled")]
    enabled: bool
}

pub struct OutputsCommand {
}

impl OutputsCommand {
    pub fn new() -> OutputsCommand {
        OutputsCommand {}
    }
}

impl MpdCommand<Vec<OutputEntry>> for OutputsCommand {
    fn handle(&self, _app: &SharedApp) -> Result<Vec<OutputEntry>, MpdError> {
        Ok(vec![
            OutputEntry {
                id: 0,
                name: String::from("Default"),
                enabled: true
            }
        ])
    }
}