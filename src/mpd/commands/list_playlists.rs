use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use library::Playlist;
use app::SharedApp;

#[derive(Debug, Serialize)]
pub struct PlaylistEntry {
    playlist: String,
    #[serde(rename = "Last-Modified")]
    last_modified: String
}

impl From<Playlist> for PlaylistEntry {
    fn from(playlist: Playlist) -> PlaylistEntry {
        PlaylistEntry {
            playlist: playlist.title,
            last_modified: "2017-12-23T17:15:13Z".to_owned()
        }
    }
}

pub struct ListPlaylistsCommand {
}

impl ListPlaylistsCommand {
    pub fn new() -> ListPlaylistsCommand {
        ListPlaylistsCommand {}
    }
}

impl MpdCommand<Vec<PlaylistEntry>> for ListPlaylistsCommand {
    fn handle(&self, app: &SharedApp) -> Result<Vec<PlaylistEntry>, MpdError> {
        let playlists = app
            .library
            .playlists
            .read()
            .unwrap()
            .iter()
            .cloned()
            .map(PlaylistEntry::from)
            .collect();
        Ok(playlists)
    }
}