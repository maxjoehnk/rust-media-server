use library::{Artist, Album, Track, Playlist};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Serialize)]
pub struct Library {
    #[serde(skip_serializing)]
    album_id: AtomicUsize,
    #[serde(skip_serializing)]
    artist_id: AtomicUsize,
    #[serde(skip_serializing)]
    track_id: AtomicUsize,
    #[serde(skip_serializing)]
    playlist_id: AtomicUsize,
    pub albums: RwLock<Vec<Album>>,
    pub artists: RwLock<Vec<Artist>>,
    pub tracks: RwLock<Vec<Track>>,
    pub playlists: RwLock<Vec<Playlist>>
}

impl Library {
    pub fn new() -> Library {
        Library {
            album_id: AtomicUsize::new(1),
            artist_id: AtomicUsize::new(1),
            track_id: AtomicUsize::new(1),
            playlist_id: AtomicUsize::new(1),
            albums: RwLock::new(vec![]),
            artists: RwLock::new(vec![]),
            tracks: RwLock::new(vec![]),
            playlists: RwLock::new(vec![])
        }
    }

    pub fn get_track(&self, id: &usize) -> Option<Track> {
        self.tracks
            .read()
            .unwrap()
            .iter()
            .cloned()
            .find(|track| track.id == Some(*id))
    }

    pub fn get_album(&self, id: &usize) -> Option<Album> {
        self.albums
            .read()
            .unwrap()
            .iter()
            .cloned()
            .find(|album| album.id == Some(*id))
    }

    pub fn add_tracks(&self, tracks: &mut Vec<Track>) {
        let tracks = tracks
            .iter()
            .cloned()
            .map(|mut track| {
                track.id = Some(self.track_id.fetch_add(1, Ordering::Relaxed));
                track
            });
        self.tracks.write().unwrap().extend(tracks);
    }

    pub fn add_albums(&self, albums: &mut Vec<Album>) {
        let albums = albums
            .iter()
            .cloned()
            .map(|mut album| {
                album.id = Some(self.album_id.fetch_add(1, Ordering::Relaxed));
                album
            });
        self.albums.write().unwrap().extend(albums);
    }

    pub fn add_album(&self, album: &mut Album) {
        album.id = Some(self.album_id.fetch_add(1, Ordering::Relaxed));
        self.albums.write().unwrap().push(album.clone());
    }

    pub fn add_artist(&self, artist: &mut Artist) {
        artist.id = Some(self.artist_id.fetch_add(1, Ordering::Relaxed));
        self.artists.write().unwrap().push(artist.clone());
    }

    pub fn search(&self, query: &'static str) -> Vec<Track> {
        self.tracks
            .read()
            .unwrap()
            .iter()
            .cloned()
            .filter(|track| track.title.contains(query))
            .collect()
    }
}