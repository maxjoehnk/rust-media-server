// Logging
use slog;
use slog_term;
use slog::Drain;
use std;

// Iron
use iron::prelude::*;
use iron::error::HttpResult;
use iron::{Handler, Listening};

use mount::Mount;
use staticfile::Static;
use std::path::Path;

use std::sync::{Arc, Mutex};

use library::Library;

mod api;

lazy_static! {
    static ref logger: slog::Logger = slog::Logger::root(
        slog_term::FullFormat::new(slog_term::PlainSyncDecorator::new(std::io::stdout()))
            .build().fuse(), o!()
    );
}

#[derive(Deserialize, Clone)]
pub struct HttpConfig {
    ip: String,
    port: i32
}

fn build_mount(library: Arc<Mutex<Library>>) -> Mount {
    let mut mount = Mount::new();
    // Frontend
    mount.mount("/", Static::new(Path::new("app/dist")));
    // Graphql Api
    // TODO
    // Rest API
    mount.mount("/api", api::build(library));
    mount
}

pub fn open(config: HttpConfig, library: Arc<Mutex<Library>>) -> HttpResult<Listening> {
    let mount = build_mount(library);
    let server = Iron::new(mount);
    let guard = server.http(format!("{}:{}", config.ip, config.port));
    info!(logger, "[HTTP] Listening on Port {}", config.port);

    guard
}