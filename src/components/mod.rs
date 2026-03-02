mod player_buttons;
mod load_image;
mod audio_player;
mod load_directory;
mod track_list;

use std::sync::Arc;
use player_buttons::{PlayButton, NextButton, PreviousButton };
use dioxus::prelude::*;
use crate::components::audio_player::{Track, MusicState, AudioPlayer};
use crate::components::track_list::TrackList;

const MAIN_CSS: &'static str = include_str!("../../assets/main.css");

#[component]
pub fn App() -> Element {
    let image_src = use_signal(|| String::new());

    let player = Arc::new(AudioPlayer::try_new().expect("xyila"));

    let tracks = use_signal(|| Vec::<Track>::new());
    let mut current_track = use_signal(|| Option::<Track>::None);
    let mut track_state = use_signal(|| MusicState::Stopped);

    let handle_track_select = move |track: Track| {
        current_track.set(Some(track.clone()));
        track_state.set(MusicState::Stopped);
    };

    let handle_play = {
        let player = player.clone();

        move |path: String| {
            player.play(&path);
        }
    };

    let handle_pause = {
        let player = player.clone();

        move |_| {

            player.pause();
        }

    };

    rsx! {
        document::Style{ { MAIN_CSS } },

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
                    "0. Title"
                },

                div {
                    class: "player-buttons-div",

                    PreviousButton {},
                    PlayButton { current_track, track_state, handle_play, handle_pause },
                    NextButton {},
                },
            }
        }
    }
}


