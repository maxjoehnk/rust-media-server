use super::super::Config;
use std::thread;
use std::time::Duration;
use library::GlobalLibrary;
use provider::ProviderInstance;

pub fn spawn(config: Config, library: GlobalLibrary) -> thread::JoinHandle<()> {
    loop {
        let pocketcasts = config.pocketcasts.clone();
        if pocketcasts.is_some() {
            let mut provider = pocketcasts.unwrap();
            provider.sync(library.clone()).unwrap();
        }
        thread::sleep(Duration::from_secs(5 * 60));
    }
}