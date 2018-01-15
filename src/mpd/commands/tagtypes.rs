use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use app::SharedApp;

pub struct TagTypesCommand {
}

#[derive(Serialize, Debug)]
pub struct TagType {
    tagtype: String
}

impl TagType {
    fn new(label: &'static str) -> TagType {
        TagType {
            tagtype: label.to_owned()
        }
    }
}

impl TagTypesCommand {
    pub fn new() -> TagTypesCommand {
        TagTypesCommand {}
    }
}

impl MpdCommand<Vec<TagType>> for TagTypesCommand {
    fn handle(&self, _app: &SharedApp) -> Result<Vec<TagType>, MpdError> {
        Ok(vec![
            TagType::new("Track"),
        ])
    }
}