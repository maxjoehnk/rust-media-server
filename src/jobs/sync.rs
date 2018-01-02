use super::super::Config;
use std::thread;
use std::time::Duration;
use library::GlobalLibrary;
use provider::ProviderInstance;
use logger::logger;
use std::sync::{Arc, Mutex};

pub fn spawn(providers: Vec<Arc<Mutex<Box<ProviderInstance + Send>>>>, library: GlobalLibrary) -> thread::JoinHandle<()> {
    thread::spawn(move|| {
        loop {
            let providers = providers.clone();
            for provider in providers {
                let mut provider = provider.lock().unwrap();
                info!(logger, "[SYNC] Syncing {} library", provider.title());
                let tracks = provider.sync(library.clone()).unwrap();
                info!(logger, "[SYNC] Synced {} tracks from {}", tracks, provider.title());
            }
            thread::sleep(Duration::from_secs(5 * 60));
        }
    })
}