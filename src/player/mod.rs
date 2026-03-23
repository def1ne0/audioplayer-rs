use std::fs::File;
use std::sync::Arc;
use std::sync::Mutex;
use std::io::BufReader;
use dioxus::prelude::{ReadableExt, Signal, WritableExt};
use rodio::{Decoder, OutputStream, Sink};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MusicState {
    Playing,
    Stopped,
}

pub struct AudioPlayer {
    _stream: OutputStream,
    sink: Arc<Mutex<Sink>>,
    on_track_end: Option<Signal<usize>>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Track {
    pub cover_src: Option<String>,
    pub path: String,
    pub name: String,
}

impl AudioPlayer {
    #[inline]
    pub fn try_new(signal: Signal<usize>) -> Option<Self> {
        let (_stream, stream_handle) = OutputStream::try_default().ok()?;
        let sink = Sink::try_new(&stream_handle).ok()?;
        Some(Self {
            _stream,
            sink:
            Arc::new(Mutex::new(sink)),
            on_track_end: Some(signal),
        })
    }

    pub fn play(&self, curr_path: &str) {
        let sink_lock = self.sink.clone();
        let signal_opt = self.on_track_end.clone();

        if let Ok(sink) = sink_lock.lock() {
            sink.stop();
        }

        if let Ok(file) = File::open(curr_path) {
            if let Ok(source) = Decoder::new(BufReader::new(file)) {
                if let Ok(sink) = self.sink.lock() {
                    sink.append(source);
                    sink.play();
                }
            }
        }

        dioxus::prelude::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                if let Ok(sink) = sink_lock.lock() {
                    if sink.empty() {
                        if let Some(mut signal) = signal_opt {
                            let val = *signal.read();
                            signal.set(val + 1);
                        }

                        break;
                    }
                }
            }
        });
    }

    pub fn pause(&self) {
        if let Ok(sink) = self.sink.lock() {
            sink.pause();
        }
    }
}