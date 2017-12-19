#[macro_use]
extern crate slog;
extern crate slog_term;

#[macro_use]
extern crate lazy_static;

use slog::Drain;

lazy_static! {
    static ref logger: slog::Logger = slog::Logger::root(
        slog_term::FullFormat::new(slog_term::PlainSyncDecorator::new(std::io::stdout()))
            .build().fuse(), o!()
    );
}

// mod mpd;

fn main() {
    //mpd::open("0.0.0.0:6600");
}
