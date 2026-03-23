use std::fs;
use base64::Engine;
use base64::engine::general_purpose;
use crate::components::audio_player::Track;
use dioxus::prelude::*;
use id3::{Tag, TagLike};
use rfd::AsyncFileDialog;

pub fn load_directory(mut tracks: Signal<Vec<Track>>) {
    spawn(async move {
       if let Some(folder) = AsyncFileDialog::new()
           .set_title("Pick music folder")
           .pick_folder()
           .await
       {
           let folder_path = folder.path().to_path_buf();
           let mut new_tracks = Vec::new();

           if let Ok(entries) = fs::read_dir(&folder_path) {
               for entry in entries.flatten() {
                   let path = entry.path();

                   if path.is_file() {
                       if let Some(ext) = path
                           .extension()
                           .and_then(|e| e.to_str())
                       {
                           if ["mp3", "flac", "wav", "vorbis"].contains(&ext.to_lowercase().as_str()) {
                               /* let name = path
                                   .file_stem()
                                   .and_then(|s| s.to_str())
                                   .unwrap_or("Unknown")
                                   .to_string();

                                */

                               let (title, img_uri) = read_mp3_metadata(&path.to_string_lossy());

                               let name = match title {
                                   Some(t) => t,
                                   None => {
                                       path.file_stem()
                                           .and_then(|s| s.to_str())
                                           .unwrap_or("Unknown")
                                           .to_string()
                                   },
                               };

                               new_tracks.push(Track { path: path.to_string_lossy().to_string(), name, cover_src: img_uri });
                           }
                       }
                   }
               }
           }

           new_tracks.sort_by(|a, b| a.name.cmp(&b.name));
           tracks.set(new_tracks);
       }
    });
}

pub fn read_mp3_metadata(path: &str) -> (Option<String>, Option<String>) {
    let mut track_title = Option::<String>::None;
    let mut cover_uri = Option::<String>::None;

    if let Ok(tag) = Tag::read_from_path(path) {
        if let Some(title) = tag.title() {
            track_title = Some(title.to_string());
        }

        if let Some(picture) = tag.pictures().next() {
            let base64_string = general_purpose::STANDARD.encode(&picture.data);
            cover_uri = Some(format!("data:{};base64,{}", picture.mime_type, base64_string));
        }
    }

    (track_title, cover_uri)
}