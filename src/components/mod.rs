mod player_buttons;
mod load_image;
mod audio_player;

use load_image::load_image;
use player_buttons::{ PlayButton, NextButton, PreviousButton };
use dioxus::prelude::*;

const MAIN_CSS: &'static str = include_str!("../../assets/main.css");

#[component]
pub fn App() -> Element {
    let image_src = use_signal(|| String::new());

    rsx! {
        document::Style{ { MAIN_CSS } },

        div {
            class: "main",

            div {
                class: "music-list",

                button {
                    onclick: load_image(image_src),
                    style: "padding: 10px 20px; margin-bottom: 20px; cursor: pointer;",
                    "Выбрать картинку"
                }
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
                    PlayButton {},
                    NextButton {},
                },
            }
        }
    }
}


