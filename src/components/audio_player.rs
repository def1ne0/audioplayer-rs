use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use rodio::{Decoder, OutputStream, Sink};

#[derive(Eq, PartialEq)]
pub enum MusicState {
    Playing,
    Stopped,
}

pub struct AudioPlayer {
    _stream: OutputStream,
    sink: Sink,
}

pub struct AudioPlayerState {
    pub player: Arc<AudioPlayer>,
    pub state: MusicState,
}

impl AudioPlayer {
    #[inline]
    pub fn try_new() -> Option<Self> {
        let (_stream, stream_handle) = OutputStream::try_default().ok()?;
        let sink = Sink::try_new(&stream_handle).ok()?;
        Some(Self { _stream, sink })
    }

    pub fn play(&self, path: &str) {
        self.sink.stop();

        if let Ok(file) = File::open(path) {
            if let Ok(source) = Decoder::new(BufReader::new(file)) {
                self.sink.append(source);
                self.sink.play();
            }
        }
    }

    pub fn pause(&self) {
        self.sink.pause();
    }
}