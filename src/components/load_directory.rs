use std::fs;
use crate::components::audio_player::Track;
use dioxus::prelude::*;
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
                           if ["mp3"].contains(&ext.to_lowercase().as_str()) {
                               let name = path
                                   .file_stem()
                                   .and_then(|s| s.to_str())
                                   .unwrap_or("Unknown")
                                   .to_string();

                               new_tracks.push(Track { path, name });
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