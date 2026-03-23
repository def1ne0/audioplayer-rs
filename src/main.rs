mod components;
pub mod file_utils;
pub mod player;
pub mod handlers;

use dioxus::desktop::{Config};
use components::App;

fn main() {
    dioxus::LaunchBuilder::new().with_cfg(Config::default()).launch(App);
}


