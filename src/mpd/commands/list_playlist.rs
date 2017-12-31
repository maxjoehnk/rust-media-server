use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use library::{GlobalLibrary, Track};
use player::GlobalPlayer;

#[derive(Debug, Serialize)]
pub struct PlaylistItem {
    file: String
}

impl From<Track> for PlaylistItem {
    fn from(track: Track) -> PlaylistItem {
        PlaylistItem {
            file: track.url
        }
    }
}

pub struct ListPlaylistCommand {
    name: String
}

impl ListPlaylistCommand {
    pub fn new(name: String) -> ListPlaylistCommand {
        ListPlaylistCommand {
            name
        }
    }
}

impl MpdCommand<Vec<PlaylistItem>> for ListPlaylistCommand {
    fn handle(&self, player: &GlobalPlayer, library: &GlobalLibrary) -> Result<Vec<PlaylistItem>, MpdError> {
        let library = library.lock().unwrap();
        let playlist = library
            .playlists
            .iter()
            .find(|playlist| playlist.title == self.name);
        match playlist {
            Some(playlist) => {
                let tracks = playlist.tracks
                    .iter()
                    .cloned()
                    .map(PlaylistItem::from)
                    .collect();
                Ok(tracks)
            },
            None => Ok(vec![])
        }
    }
}