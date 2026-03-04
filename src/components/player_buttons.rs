use crate::components::audio_player::{MusicState, Track};
use dioxus::prelude::*;

#[component]
pub fn PlayButton(
    mut track_state: Signal<MusicState>,
    mut current_track: Signal<Option<Track>>,
    handle_play: EventHandler<String>,
    handle_pause: EventHandler<()>,
) -> Element {
    let toggle_play = move |_| {
        let current_state = *track_state.read();

        match current_state {
            MusicState::Playing => {
                handle_pause(());
                track_state.set(MusicState::Stopped);
            },
            MusicState::Stopped => {
                if let Some(track) = current_track.read().clone() {
                    if let Some(path) = track.path.to_str() {
                        handle_play(path.to_string());
                        track_state.set(MusicState::Playing);
                    }
                }
            }
        }
    };

    rsx! {
        button {
            class: "player-buttons",

            onclick: toggle_play,

            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "50",
                height: "50",
                view_box: "0 0 28 28",

                path {
                    fill: "#FFFFFF",
                    d: "M12.766 9.278a1.5 1.5 0 0 0-2.266 1.29v6.864a1.5 1.5 0 0 0 2.266 1.29l6.505-3.862a1 1 0 0 0 0-1.72l-6.505-3.862ZM2 14C2 7.373 7.373 2 14 2s12 5.373 12 12s-5.373 12-12 12S2 20.627 2 14ZM14 3.5C8.201 3.5 3.5 8.201 3.5 14S8.201 24.5 14 24.5S24.5 19.799 24.5 14S19.799 3.5 14 3.5Z",
                }
            }
        }
    }
}

#[component]
pub fn NextButton(
    handle_next: EventHandler<()>,
) -> Element {
    rsx! {
        button {
            class: "player-buttons",
            onclick: move |_| handle_next(()),

            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "50",
                height: "50",
                view_box: "0 0 48 48",

                path {
                    fill: "#FFFFFF",
                    d: "M40.75 6c-.69 0-1.25.56-1.25 1.25v33.5a1.25 1.25 0 0 0 2.5 0V7.25C42 6.56 41.44 6 40.75 6ZM6 9.256c0-2.615 2.931-4.16 5.088-2.68l21.504 14.743c1.883 1.29 1.883 4.07 0 5.36L11.088 41.424C8.93 42.9 6 41.357 6 38.743V9.255Z",
                }
            }
        }
    }
}

#[component]
pub fn PreviousButton(
    handle_prev: EventHandler<()>,
) -> Element {
    rsx! {
        button {
            class: "player-buttons",
            onclick: move |_| handle_prev(()),

            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "50",
                height: "50",
                view_box: "0 0 48 48",

                path {
                    fill: "#FFFFFF",
                    d: "M7.25 6c.69 0 1.25.56 1.25 1.25v33.5a1.25 1.25 0 1 1-2.5 0V7.25C6 6.56 6.56 6 7.25 6ZM42 9.256c0-2.615-2.93-4.16-5.088-2.68L15.408 21.318c-1.883 1.29-1.883 4.07 0 5.36l21.504 14.744C39.07 42.9 42 41.357 42 38.743V9.255Z",
                }
            }
        }
    }
}
