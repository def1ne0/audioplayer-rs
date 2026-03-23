use dioxus::prelude::*;
use std::sync::Arc;
use crate::player::{AudioPlayer, MusicState, Track};

pub fn handle_play(player: Signal<Arc<AudioPlayer>>) -> EventHandler<String> {
    EventHandler::new(
        move |path: String| {
            player.read().play(&path);
        }
    )
}

pub fn handle_pause(player: Signal<Arc<AudioPlayer>>) -> impl Fn() + 'static {
    move || {
        player.read().pause();
    }
}

pub fn handle_track_select(
    player: Signal<Arc<AudioPlayer>>,
    tracks: Signal<Vec<Track>>,
    mut current_track: Signal<Option<Track>>,
    mut track_state: Signal<MusicState>,
    mut title: Signal<String>,
    mut image_src: Signal<Option<String>>,
    mut current_index: Signal<usize>,
    track: Track,
) {
    current_track.set(Some(track.clone()));
    title.set(track.name.clone());
    image_src.set(track.cover_src.clone());
    current_index.set(tracks.read().iter().position(|t| *t == track).unwrap());

    println!("selected idx: {}", current_index.read());
    player.read().play(&track.path);
    track_state.set(MusicState::Playing);
}

pub fn handle_next(
    player: Signal<Arc<AudioPlayer>>,
    tracks: Signal<Vec<Track>>,
    current_track: Signal<Option<Track>>,
    track_state: Signal<MusicState>,
    title: Signal<String>,
    image_src: Signal<Option<String>>,
    mut current_index: Signal<usize>,
) -> impl FnMut() + 'static {
    move || {
        let all_tracks = tracks.read().clone();
        let val = (*current_index.read() + 1) % all_tracks.len();
        current_index.set(val);
        handle_track_select(player, tracks, current_track, track_state, title, image_src, current_index, all_tracks[val].clone());
    }
}

pub fn handle_previous(
    player: Signal<Arc<AudioPlayer>>,
    tracks: Signal<Vec<Track>>,
    current_track: Signal<Option<Track>>,
    track_state: Signal<MusicState>,
    title: Signal<String>,
    image_src: Signal<Option<String>>,
    mut current_index: Signal<usize>,
) -> impl FnMut() + 'static{
    move || {
        let all_tracks = tracks.read().clone();
        let curr_idx = *current_index.read();

        let val = match curr_idx {
            0 => all_tracks.len() - 1,
            _ => curr_idx  - 1,
        };

        current_index.set(val);
        handle_track_select(player, tracks, current_track, track_state, title, image_src, current_index, all_tracks[val].clone());
    }
}