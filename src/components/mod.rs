mod player_buttons;
mod track_list;

use std::sync::Arc;
use player_buttons::{NextButton, PlayButton, PreviousButton};
use dioxus::prelude::*;
use crate::player::{AudioPlayer, MusicState, Track};
use crate::components::track_list::TrackList;
use crate::handlers::{handle_next, handle_pause, handle_play, handle_previous, handle_track_select};


#[component]
pub fn App() -> Element {
    let track_ended_signal = use_signal(|| 0_usize);

    let mut image_src = use_signal(|| Option::<String>::None);

    let player = use_signal(|| Arc::new(AudioPlayer::try_new(track_ended_signal).expect("Error while2 creating audioplayer")));

    let tracks = use_signal(|| Vec::<Track>::new());
    let mut current_track = use_signal(|| Option::<Track>::None);
    let track_state = use_signal(|| MusicState::Stopped);
    let mut title = use_signal(|| String::from("Unknown"));
    let mut current_index = use_signal(|| 0_usize);

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
            image_src.set(all_tracks[idx].cover_src.clone());
            player.read().play(&all_tracks[idx].path);
       });
    });

    rsx! {
        style { "{include_str!(\"../../assets/main.css\")}" },

        div {
            class: "main",

            TrackList {
                tracks,
                selected_track: current_track,
                on_select: move |track: Track| {
                    handle_track_select(player, tracks, current_track, track_state, title, image_src, current_index, track);
                },
            },

            div {
                class: "player-menu",

                if let Some(src) = image_src.read().clone() {
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

                    PreviousButton {
                        handle_prev:handle_previous(player, tracks, current_track, track_state, title, image_src, current_index)
                    },
                    PlayButton {
                        current_track,
                        track_state,
                        handle_play: handle_play(player),
                        handle_pause: handle_pause(player)
                    },
                    NextButton {
                        handle_next: handle_next(player, tracks, current_track, track_state, title, image_src, current_index)
                    },
                },
            }
        }
    }
}


