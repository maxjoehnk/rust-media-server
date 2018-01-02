use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use mpd::song::MpdSong;
use library::GlobalLibrary;
use player::GlobalPlayer;
use provider::SharedProviders;

pub struct ListPlaylistInfoCommand {
    name: String
}

impl ListPlaylistInfoCommand {
    pub fn new(name: String) -> ListPlaylistInfoCommand {
        ListPlaylistInfoCommand {
            name
        }
    }
}

impl MpdCommand<Vec<MpdSong>> for ListPlaylistInfoCommand {
    fn handle(&self, _player: &GlobalPlayer, library: &GlobalLibrary, _providers: &SharedProviders) -> Result<Vec<MpdSong>, MpdError> {
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
                    .map(MpdSong::from)
                    .collect();
                Ok(tracks)
            },
            None => Ok(vec![])
        }
    }
}