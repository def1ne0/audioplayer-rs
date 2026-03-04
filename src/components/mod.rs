mod player_buttons;
mod load_image;
mod audio_player;
mod load_directory;
mod track_list;

use std::sync::{Arc};
use player_buttons::{PlayButton, NextButton, PreviousButton };
use dioxus::prelude::*;
use crate::components::audio_player::{Track, MusicState, AudioPlayer};
use crate::components::track_list::TrackList;

static MAIN_CSS: Asset = asset!("../../assets/main.css");

#[component]
pub fn App() -> Element {
    let track_ended_signal = use_signal(|| 0_usize);

    let image_src = use_signal(|| String::new());

    let player = use_signal(|| Arc::new(AudioPlayer::try_new(track_ended_signal).expect("Error while2 creating audioplayer")));

    let tracks = use_signal(|| Vec::<Track>::new());
    let mut current_track = use_signal(|| Option::<Track>::None);
    let mut track_state = use_signal(|| MusicState::Stopped);
    let mut title = use_signal(|| String::new());
    let mut current_index = use_signal(|| 0_usize);

    let handle_play = {
        let player = player.clone();

        move |path: String| {
            player.read().play(&path);
        }
    };

    use_effect(move || {
        let count = *track_ended_signal.read();

        if count == 0 {
            return;
        };

        spawn(async move {
            let mut idx = current_index.read().clone();
            let all_tracks = tracks.read().clone();

            idx = (idx + 1) % all_tracks.len();
            current_index.set(idx);
            current_track.set(Some(all_tracks[idx].clone()));

            println!("idx: {}, {}", idx, all_tracks[idx].name);

            if all_tracks.is_empty() {
                return;
            }

            title.set(all_tracks[idx].name.clone());
            player.read().play(all_tracks[idx].path.to_str().unwrap());
       });
    });

    let handle_pause = {
        let player = player.clone();

        move || {
            player.read().pause();
        }
    };

    let mut handle_track_select = {
        let player = player.clone();

        move |track: Track| {
            current_track.set(Some(track.clone()));
            title.set(
                current_track.read().as_ref()
                    .map(|t| t.name.to_string())
                    .unwrap_or(String::from("Unknown"))
            );
            current_index.set(tracks.read().iter().position(|t| *t == track).unwrap());
            println!("selected idx: {}", current_index.read());
            player.read().play(track.path.to_str().unwrap());
            track_state.set(MusicState::Playing);
        }
    };

    let handle_next = move || {
        let all_tracks = tracks.read().clone();
        let val = (*current_index.read() + 1) % all_tracks.len();
        current_index.set(val);
        handle_track_select(all_tracks[val].clone());
    };

    let handle_prev = move || {
        let all_tracks = tracks.read().clone();
        let curr_idx = *current_index.read();

        let val = match curr_idx {
            0 => all_tracks.len() - 1,
            _ => curr_idx  - 1,
        };

        current_index.set(val);
        handle_track_select(all_tracks[val].clone());
    };

    rsx! {
        Stylesheet { href: MAIN_CSS },

        div {
            class: "main",

            TrackList {
                tracks,
                selected_track: current_track,
                on_select: handle_track_select,
            },

            div {
                class: "player-menu",

                if !image_src.read().is_empty() {
                    img {
                        src: image_src,
                        width: 200,
                        height: 200,
                    },
                } else {
                    div {
                        class: "null-music",
                    }
                },

                input {
                    class: "music-duration",
                    r#type: "range",
                    min: "0",
                    max: "100",
                    value: "0",
                    step: "1",
                    disabled: true,
                },

                p {
                    class: "music-title",
                    "{title}"
                },

                div {
                    class: "player-buttons-div",

                    PreviousButton { handle_prev },
                    PlayButton { current_track, track_state, handle_play, handle_pause },
                    NextButton { handle_next },
                },
            }
        }
    }
}


