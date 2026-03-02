use dioxus::prelude::*;
use crate::components::audio_player::Track;
use crate::components::load_directory::load_directory;

#[component]
pub fn TrackList(
    mut tracks: Signal<Vec<Track>>,
    selected_track: Signal<Option<Track>>,
    on_select: EventHandler<Track>,
) -> Element {

    let tracks_vec: Vec<Track> = tracks.read().iter().cloned().collect();

    rsx! {
        div {
            class: "music-list",

            if tracks.read().is_empty() {
                div {
                    button {
                        onclick: move |_| load_directory(tracks),
                        style: "padding: 10px 20px; margin-bottom: 20px; cursor: pointer;",
                        "Pick music folder",
                    }
                }
            } else {
                for track in tracks_vec {
                    TrackItem {
                        track: track.clone(),
                        is_selected: selected_track.read().as_ref()
                            .map(|t| t.path == track.path)
                            .unwrap_or(false),
                        on_select: move |_| on_select.call(track.clone()),
                    }
                }
            },
        }
    }
}

#[component]
pub fn TrackItem(
    track: Track,
    is_selected: bool,
    on_select: EventHandler<Track>,
) -> Element {
    rsx! {
        div {
            class: "track-item",
            onclick: move |_| on_select.call(track.clone()),
            span { class: "track-name", "{track.name}" }
        }
    }
}
