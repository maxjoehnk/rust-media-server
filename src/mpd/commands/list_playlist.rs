use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use library::{GlobalLibrary, Track};
use player::GlobalPlayer;
use provider::SharedProviders;
use rayon::prelude::*;

#[derive(Debug, Serialize)]
pub struct PlaylistItem {
    file: String
}

impl From<Track> for PlaylistItem {
    fn from(track: Track) -> PlaylistItem {
        PlaylistItem {
            file: track.uri
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
    fn handle(&self, _player: &GlobalPlayer, library: &GlobalLibrary, providers: &SharedProviders) -> Result<Vec<PlaylistItem>, MpdError> {
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
                    .par_iter()
                    .map(|uri| library.resolve_track(providers.clone(), uri))
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