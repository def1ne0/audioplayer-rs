mod components;
mod utilities;

use dioxus::desktop::{Config};
use components::App;

fn main() {
    dioxus::LaunchBuilder::new().with_cfg(Config::default()).launch(App);
}


