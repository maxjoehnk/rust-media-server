use super::super::Config;
use std::thread;
use std::time::Duration;
use library::GlobalLibrary;
use provider::ProviderInstance;
use logger::logger;

pub fn spawn(config: Config, library: GlobalLibrary) -> thread::JoinHandle<()> {
    thread::spawn(move|| {
        loop {
            let threads = vec![
                spawn_pocketcasts(config.clone(), library.clone()),
                spawn_soundcloud(config.clone(), library.clone())
            ];
            for handle in threads {
                let _ = handle.join();
            }
            thread::sleep(Duration::from_secs(5 * 60));
        }
    })
}

fn spawn_pocketcasts(config: Config, library: GlobalLibrary) -> thread::JoinHandle<()> {
    thread::spawn(move|| {
        let pocketcasts = config.pocketcasts.clone();
        if pocketcasts.is_some() {
            let mut provider = pocketcasts.unwrap();
            info!(logger, "[SYNC] Syncing Pocketcasts Library");
            provider.sync(library).unwrap();
        }
    })
}

fn spawn_soundcloud(config: Config, library: GlobalLibrary) -> thread::JoinHandle<()> {
    thread::spawn(move|| {
        let soundcloud = config.soundcloud.clone();
        if soundcloud.is_some() {
            let mut provider = soundcloud.unwrap();
            info!(logger, "[SYNC] Syncing Soundcloud Library");
            provider.sync(library).unwrap();
        }
    })
}