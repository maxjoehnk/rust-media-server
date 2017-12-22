use gstreamer as gst;
use player::Queue;
use gstreamer::MessageView;
use std::thread;
use gstreamer::prelude::*;

#[derive(Debug, Clone)]
pub struct Player {
    pub queue: Queue
}

impl Player {
    pub fn new() -> Player {
        Player {
            queue: Queue::new()
        }
    }

    pub fn play(&mut self) {
        let decoder = gst::ElementFactory::make("uridecodebin", None).expect("uridecodebin");
        let sink = gst::ElementFactory::make("autoaudiosink", None).expect("autoaudiosink");
        let pipeline = gst::Pipeline::new(None);

        pipeline.add(&decoder).expect("decoder");
        pipeline.add(&sink).expect("sink");

        let sink_pad = sink.get_static_pad("sink").expect("sink_pad");
        decoder.connect_pad_added(move |_el: &gst::Element, pad: &gst::Pad| {
            pad.link(&sink_pad);
        });

        match self.queue.current() {
            Some(track) => {
                println!("Playing {:?}", track);
                decoder.set_property_from_str("uri", track.url.as_str());
            },
            None => {}
        }

        let bus = pipeline.get_bus().unwrap();

        let ret = pipeline.set_state(gst::State::Playing);

        println!("{:?}", ret);

        assert_ne!(ret, gst::StateChangeReturn::Failure);

        loop {
            let msg = match bus.timed_pop(gst::CLOCK_TIME_NONE) {
                None => break,
                Some(msg) => msg,
            };

            match msg.view() {
                MessageView::Eos(..) => {
                    println!("arrived at the end {:?}", self.queue);
                    let next = self.queue.next();
                    match next {
                        Some(track) => {
                            pipeline.set_state(gst::State::Null);
                            println!("Playing {:?}", track);
                            decoder.set_property_from_str("uri", track.url.as_str());

                            let ret = pipeline.set_state(gst::State::Playing);

                            println!("{:?}", ret);

                            assert_ne!(ret, gst::StateChangeReturn::Failure);
                        },
                        None => {
                            println!("We're out of tracks");
                        }
                    }
                },
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
        }
    }
}