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
            file: track.path
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
    fn handle(&self, _player: &GlobalPlayer, library: &GlobalLibrary) -> Result<Vec<PlaylistItem>, MpdError> {
        let playlists = library
            .playlists
            .read()
            .unwrap();
        let playlist = playlists
            .iter()
            .find(|playlist| playlist.title == self.name);
        match playlist {
            Some(playlist) => {
                let tracks = playlist.tracks
                    .iter()
                    .map(|id| library.get_track(id))
                    .filter(|track| track.is_some())
                    .map(|track| track.unwrap())
                    .map(PlaylistItem::from)
                    .collect();
                Ok(tracks)
            },
            None => Ok(vec![])
        }
    }
}