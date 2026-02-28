mod components;

use dioxus::desktop::{ Config, WindowBuilder };
use components::App;

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(Config::default()
            .with_window(
                WindowBuilder::default()
                    .with_decorations(false)
                    .with_title("audioplayer")
            )
        )
        .launch(App)
}


