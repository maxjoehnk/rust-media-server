use mpd::error::MpdError;
use mpd::commands::MpdCommand;
use mpd::commands::list_playlists::PlaylistEntry;
use mpd::song::MpdSong;
use provider::Explorer;
use app::SharedApp;
use library::SharedLibrary;

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

    fn get_playlists(&self, library: &SharedLibrary) -> Vec<PlaylistEntry> {
        library
            .playlists
            .read()
            .unwrap()
            .iter()
            .cloned()
            .map(PlaylistEntry::from)
            .collect()
    }
}

type ListInfoResponse = (Vec<PathItem>, Vec<PlaylistEntry>, Vec<MpdSong>);

impl MpdCommand<ListInfoResponse> for ListInfoCommand {
    fn handle(&self, app: &SharedApp) -> Result<ListInfoResponse, MpdError> {
        match self.path {
            None => {
                let explorer = Explorer::new(app.providers.to_vec());
                let folders = explorer
                    .items()
                    .unwrap()
                    .folders
                    .iter()
                    .map(|folder| {
                        PathItem {
                            directory: folder.clone()
                        }
                    })
                    .collect();
                let playlists = self.get_playlists(&app.library);
                Ok((folders, playlists, vec![]))
            }
            Some(ref path) => {
                let mut explorer = Explorer::new(app.providers.to_vec());
                explorer.navigate_absolute(path.to_owned());
                let path = explorer.path();
                let folder = explorer.items().unwrap();
                let folders = folder
                    .folders
                    .iter()
                    .map(|folder| {
                        PathItem {
                            directory: format!("{}{}", path, folder)
                        }
                    })
                    .collect();
                let items = folder
                    .items
                    .iter()
                    .filter(|item| item.track.is_some())
                    .cloned()
                    .map(|item| item.track.unwrap())
                    .map(MpdSong::from)
                    .collect();
                let playlists = folder
                    .items
                    .iter()
                    .filter(|item| item.playlist.is_some())
                    .cloned()
                    .map(|item| item.playlist.unwrap())
                    .map(PlaylistEntry::from)
                    .collect();
                Ok((folders, playlists, items))
            }
        }
    }
}