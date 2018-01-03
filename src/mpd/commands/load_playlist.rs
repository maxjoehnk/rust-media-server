use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use library::GlobalLibrary;
use player::GlobalPlayer;
use provider::SharedProviders;
use rayon::prelude::*;

pub struct LoadPlaylistCommand {
    name: String
}

impl LoadPlaylistCommand {
    pub fn new(name: String) -> LoadPlaylistCommand {
        LoadPlaylistCommand {
            name
        }
    }
}

impl MpdCommand<()> for LoadPlaylistCommand {
    fn handle(&self, player: &GlobalPlayer, library: &GlobalLibrary, providers: &SharedProviders) -> Result<(), MpdError> {
        let tracks = library
            .playlists
            .read()
            .unwrap()
            .iter()
            .find(|playlist| playlist.title == self.name)
            .unwrap()
            .tracks
            .par_iter()
            .map(|uri| library.resolve_track(providers.clone(), uri))
            .filter(|track| track.is_some())
            .map(|track| track.unwrap())
            .collect();
        let mut player = player.lock().unwrap();
        player.queue.add_multiple(tracks);
        Ok(())
    }
}