mod player_buttons;
mod track_list;

use std::rc::Rc;
use player_buttons::{NextButton, PlayButton, PreviousButton};
use dioxus::prelude::*;
use crate::player::{AudioPlayer, MusicState, track::Track, player_context::PlaylistContext};
use crate::components::track_list::TrackList;
use crate::hooks::use_track_autoplay;

#[component]
pub fn App() -> Element {
    let track_ended_signal = use_signal(|| 0_usize);

    let image_src = use_signal(|| Option::<String>::None);
    let player = use_signal(|| Rc::new(AudioPlayer::try_new(track_ended_signal).expect("Error while creating audio player")));
    let tracks = use_signal(Vec::<Track>::new);
    let current_track = use_signal(|| Option::<Track>::None);
    let track_state = use_signal(|| MusicState::Stopped);
    let title = use_signal(|| String::from("Unknown"));
    let current_index = use_signal(|| 0_usize);

    let mut player_ctx = PlaylistContext {
        player,
        tracks,
        current_track,
        track_state,
        title,
        image_src,
        current_index,
    };

    use_track_autoplay(player_ctx, track_ended_signal);

    rsx! {
        style { "{include_str!(\"../../assets/main.css\")}" },

        div {
            class: "main",

            TrackList {
                tracks,
                selected_track: current_track,

                on_select: {
                    player_ctx.on_track_select()
                },
            },

            div {
                class: "player-menu",

                if let Some(src) = player_ctx.image_src.read().clone() {
                    img {
                        src: src,
                        width: 200,
                        height: 200,
                    },
                } else {
                    div {
                        class: "null-music",
                    },
                },
                input {
                    class: "music-duration",
                    r#type: "range",
                    min: "0",
                    max: "1",
                    value: "1",
                    step: "0.01",
                    oninput: player_ctx.on_volume_changed(),
                },

                p {
                    class: "music-title",
                    "{title}"
                },

                div {
                    class: "player-buttons-div",

                    PreviousButton {
                        handle_prev: player_ctx.on_previous(),
                    },
                    PlayButton {
                        current_track,
                        track_state,
                        handle_play: player_ctx.on_play(),
                        handle_pause: player_ctx.on_pause(),
                    },
                    NextButton {
                        handle_next: player_ctx.on_next(),
                    },
                },
            }
        }
    }
}


