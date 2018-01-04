use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use mpd::song::MpdSong;
use rayon::prelude::*;
use app::SharedApp;

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
    fn handle(&self, app: &SharedApp) -> Result<Vec<MpdSong>, MpdError> {
        let playlists = app
            .library
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
                    .cloned()
                    .map(MpdSong::from)
                    .collect();
                Ok(tracks)
            },
            None => Ok(vec![])
        }
    }
}