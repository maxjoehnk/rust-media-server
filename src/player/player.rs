use gstreamer as gst;
use player::{Queue, GlobalPlayer};
use gstreamer::MessageView;
use std::thread;
use std::time::Duration;
use gstreamer::prelude::*;
use library::Track;

#[derive(Debug, Clone)]
pub struct Player {
    playing: bool,
    pub queue: Queue,
    backend: GstBackend
}

impl Player {
    pub fn new() -> Player {
        let player = Player {
            playing: false,
            queue: Queue::new(),
            backend: GstBackend::new()
        };

        player
    }

    pub fn play(&mut self) {
        self.playing = true;
        match self.queue.current() {
            Some(track) => {
                self.select_track(&track);
            },
            None => {}
        }
    }

    pub fn next(&mut self) {
        match self.queue.current() {
            Some(track) => {
                self.select_track(&track);
            },
            None => {}
        }
    }

    pub fn get_backend(&self) -> &GstBackend {
        &self.backend
    }

    fn select_track(&self, track: &Track) {
        self.backend.set_track(track, self.playing);
    }
}

#[derive(Debug, Clone)]
struct GstBackend {
    pipeline: gst::Pipeline,
    decoder: gst::Element,
    sink: gst::Element
}

impl GstBackend {
    fn new() -> GstBackend {
        let player = GstBackend {
            pipeline: gst::Pipeline::new(None),
            decoder: gst::ElementFactory::make("uridecodebin", None).expect("uridecodebin"),
            sink: gst::ElementFactory::make("autoaudiosink", None).expect("autoaudiosink")
        };

        player.pipeline.add(&player.decoder).expect("add decoder to pipeline");
        player.pipeline.add(&player.sink).expect("add sink to pipeline");

        let sink_pad = player.sink.get_static_pad("sink").expect("audio sink_pad");
        player.decoder.connect_pad_added(move |_el: &gst::Element, pad: &gst::Pad| {
            pad.link(&sink_pad);
        });

        player
    }

    fn get_bus(&self) -> gst::Bus {
        self.pipeline.get_bus().unwrap()
    }

    fn set_track(&self, track: &Track, playing: bool) {
        println!("Selecting {:?}", track);
        self.pipeline.set_state(gst::State::Null);
        self.decoder.set_property_from_str("uri", track.url.as_str());

        let state = match playing {
            true => gst::State::Playing,
            false => gst::State::Paused
        };
        let ret = self.pipeline.set_state(state);

        assert_ne!(ret, gst::StateChangeReturn::Failure);
    }
}

pub fn main_loop(player: GlobalPlayer) -> thread::JoinHandle<()> {
    thread::spawn(move|| {
        loop {
            let mut player = player.lock().unwrap();
            let bus = player.get_backend().get_bus();

            let msg = match bus.timed_pop(gst::CLOCK_TIME_NONE) {
                None => break,
                Some(msg) => msg,
            };

            match msg.view() {
                MessageView::Eos(..) => player.next(),
                MessageView::Error(err) => {
                    println!(
                        "Error from {}: {} ({:?})",
                        msg.get_src().unwrap().get_path_string(),
                        err.get_error(),
                        err.get_debug()
                    );
                    break;
                },
                _ => (),
            }
            thread::sleep(Duration::from_millis(100));
        }
    })
}