use dioxus::prelude::*;
use crate::player::player_context::PlayerContext;

// When the track is ended, it accepts the signal and plays the next track.
pub fn use_track_autoplay(mut ctx: PlayerContext, ended_signal: Signal<usize>) {
    use_effect(move || {
        let count = *ended_signal.read();

        if count == 0 {
            return;
        };

        spawn(async move {
            let mut idx = *ctx.current_index.read();
            let all_tracks = ctx.tracks.read().clone();

            idx = (idx + 1) % all_tracks.len();
            ctx.current_index.set(idx);
            ctx.current_track.set(Some(all_tracks[idx].clone()));

            #[cfg(debug_assertions)]
            println!("idx: {}, {}", idx, all_tracks[idx].name);

            if all_tracks.is_empty() {
                return;
            }

            ctx.title.set(all_tracks[idx].name.clone());
            ctx.image_src.set(all_tracks[idx].cover_src.clone());
            ctx.player.read().play(&all_tracks[idx].path);
        });
    });

}