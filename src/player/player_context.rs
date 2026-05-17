use std::rc::Rc;
use dioxus::prelude::*;
use crate::player::{AudioPlayer, MusicState};
use crate::player::track::Track;

#[derive(Clone, Copy)]
pub struct PlayerContext {
    pub player: Signal<Rc<AudioPlayer>>,
    pub tracks: Signal<Vec<Track>>,
    pub current_track: Signal<Option<Track>>,
    pub track_state: Signal<MusicState>,
    pub title: Signal<String>,
    pub image_src: Signal<Option<String>>,
    pub current_index: Signal<usize>,
}

impl PlayerContext {

    // Our main private function to set some states of components,
    // if user has selected the song.
    fn apply_track_select(
        &mut self,
        track: &Track,
    ) {
        self.current_track.set(Some(track.clone()));
        self.title.set(track.name.clone());
        self.image_src.set(track.cover_src.clone());
        self.current_index.set(self.tracks.read().iter().position(|t| *t == *track).unwrap());

        #[cfg(debug_assertions)]
        println!("selected idx: {}", self.current_index.read());

        self.player.read().play(&track.path);
        self.track_state.set(MusicState::Playing);
    }

    // Handles the song selected and returns the closure
    // to on_select attribute of button.
    pub fn on_track_select(
        &mut self,
    ) -> impl FnMut(Track) + 'static {
        // Need to copy this, because move occurs.
        let mut ctx = *self;

        move |track: Track| {
            ctx.apply_track_select(&track);
        }
    }

    // Handles the song playing and returns the closure
    // to on_select attribute of the button.
    pub fn on_play(
        &self,
    ) -> impl Fn(String) + 'static {
        let ctx = *self;
        
        move |path: String| {
            ctx.player.read().play(&path);
        }
    }

    // Handles the song paused and returns the closure
    // to on_select attribute of the button.
    pub fn on_pause(
        &self,
    ) -> impl Fn() + 'static {
        let ctx = *self;
        
        move || {
            ctx.player.read().pause();
        }
    }

    // Handles the next song selected and returns the closure
    // to on_select attribute of the button.
    pub fn on_next(
        &mut self,
    ) -> impl FnMut() + 'static {
        let mut ctx = *self;
        
        move || {
            let all_tracks = ctx.tracks.read().clone();
            let val = (*ctx.current_index.read() + 1) % all_tracks.len();
            ctx.current_index.set(val);
            ctx.apply_track_select(&all_tracks[val]);
        }
    }

    // Handles the previous song selected and returns the closure
    // to on_select attribute of the button.
    pub fn on_previous(
        &mut self,
    ) -> impl FnMut() + 'static {
        let mut ctx = *self;
        
        move || {
            let all_tracks = ctx.tracks.read().clone();
            let curr_idx = *ctx.current_index.read();

            let val = match curr_idx {
                0 => all_tracks.len() - 1,
                _ => curr_idx  - 1,
            };

            ctx.current_index.set(val);
            ctx.apply_track_select(&all_tracks[val]);
        }
    }
}