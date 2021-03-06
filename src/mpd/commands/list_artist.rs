use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use library::Artist;
use app::SharedApp;

#[derive(Debug, Serialize)]
pub struct MpdArtist {
    #[serde(rename = "Artist")]
    artist: String
}

impl From<Artist> for MpdArtist {
    fn from(artist: Artist) -> MpdArtist {
        MpdArtist {
            artist: artist.name
        }
    }
}

pub struct ListArtistCommand {}

impl ListArtistCommand {
    pub fn new() -> ListArtistCommand {
        ListArtistCommand {}
    }
}

impl MpdCommand<Vec<MpdArtist>> for ListArtistCommand {
    fn handle(&self, app: &SharedApp) -> Result<Vec<MpdArtist>, MpdError> {
        let mut artists: Vec<MpdArtist> = app
            .library
            .artists
            .read()
            .unwrap()
            .iter()
            .cloned()
            .map(MpdArtist::from)
            .collect();
        let unknown = MpdArtist {
            artist: String::from("[unknown]")
        };
        artists.insert(0, unknown);
        Ok(artists)
    }
}