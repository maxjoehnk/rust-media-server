use bus::SharedBus;
use player::SharedPlayer;
use library::SharedLibrary;
use provider::SharedProviders;
use std::sync::Arc;

pub struct App {
    pub bus: SharedBus,
    pub player: SharedPlayer,
    pub library: SharedLibrary,
    pub providers: SharedProviders
}

pub type SharedApp = Arc<App>;