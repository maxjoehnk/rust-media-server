use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use library::{GlobalLibrary, Track};
use player::GlobalPlayer;
use mpd::commands::list_playlists::PlaylistEntry;

#[derive(Serialize)]
pub struct PathItem {
    directory: String
}

pub struct ListInfoCommand {
    path: Option<String>
}

impl ListInfoCommand {
    pub fn new(path: String) -> ListInfoCommand {
        ListInfoCommand {
            path: if path == "" { None } else { Some(path) }
        }
    }

    fn get_playlists(&self, library: &GlobalLibrary) -> Vec<PlaylistEntry> {
        library.lock()
            .unwrap()
            .playlists
            .iter()
            .cloned()
            .map(PlaylistEntry::from)
            .collect()
    }
}

impl MpdCommand<(Vec<PathItem>, Vec<PlaylistEntry>)> for ListInfoCommand {
    fn handle(&self, player: &GlobalPlayer, library: &GlobalLibrary) -> Result<(Vec<PathItem>, Vec<PlaylistEntry>), MpdError> {
        match self.path {
            None => {
                let folders = vec![
                    PathItem {
                        directory: String::from("Pocketcasts")
                    },
                    PathItem {
                        directory: String::from("Soundcloud")
                    }
                ];
                let playlists = self.get_playlists(library);
                Ok((folders, playlists))
            },
            Some(ref path) => {
                Ok((vec![], vec![]))
            }
        }
    }
}