use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MusicState {
    Playing,
    Stopped,
}

pub struct AudioPlayer {
    _stream: OutputStream,
    sink: Sink,

}

#[derive(Clone, PartialEq, Eq)]
pub struct Track {
    pub path: std::path::PathBuf,
    pub name: String,
}

impl AudioPlayer {
    #[inline]
    pub fn try_new() -> Option<Self> {
        let (_stream, stream_handle) = OutputStream::try_default().ok()?;
        let sink = Sink::try_new(&stream_handle).ok()?;
        Some(Self { _stream, sink })
    }

    pub fn play(&self, curr_path: &str) {
        self.sink.stop();

        if let Ok(file) = File::open(curr_path) {
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