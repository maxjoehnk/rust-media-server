use super::super::Config;
use std::thread;
use std::time::Duration;
use library::GlobalLibrary;
use provider::ProviderInstance;
use logger::logger;

pub fn spawn(config: Config, library: GlobalLibrary) -> thread::JoinHandle<()> {
    thread::spawn(move|| {
        loop {
            sync_pocketcasts(config.clone(), library.clone());
            sync_soundcloud(config.clone(), library.clone());
            thread::sleep(Duration::from_secs(5 * 60));
        }
    })
}

fn sync_pocketcasts(config: Config, library: GlobalLibrary) {
    let pocketcasts = config.pocketcasts.clone();
    if pocketcasts.is_some() {
        let mut provider = pocketcasts.unwrap();
        info!(logger, "[SYNC] Syncing Pocketcasts library");
        let tracks = provider.sync(library).unwrap();
        info!(logger, "[SYNC] Synced {} tracks from Pocketcasts", tracks);
    }
}

fn sync_soundcloud(config: Config, library: GlobalLibrary) {
    let soundcloud = config.soundcloud.clone();
    if soundcloud.is_some() {
        let mut provider = soundcloud.unwrap();
        info!(logger, "[SYNC] Syncing Soundcloud library");
        let tracks = provider.sync(library).unwrap();
        info!(logger, "[SYNC] Synced {} tracks from Soundcloud", tracks);
    }
}