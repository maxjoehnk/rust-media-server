use std::thread;
use std::time::Duration;
use library::GlobalLibrary;
use provider::{ProviderInstance, SharedProviders};
use logger::logger;
use std::sync::{Arc, Mutex};

pub fn spawn(providers: SharedProviders, library: GlobalLibrary) -> thread::JoinHandle<()> {
    thread::spawn(move|| {
        loop {
            let providers = providers.clone();
            for provider in providers {
                let mut provider = provider.write().unwrap();
                info!(logger, "[SYNC] Syncing {} library", provider.title());
                match provider.sync(library.clone()) {
                    Ok(result) => info!(logger, "[SYNC] Synced {} tracks, {} albums, {} artist and {} playlists from {}", result.tracks, result.albums, result.artists, result.playlists, provider.title()),
                    Err(err) => error!(logger, "[SYNC] Error syncing {}: {:?}", provider.title(), err)
                }
            }
            thread::sleep(Duration::from_secs(5 * 60));
        }
    })
}