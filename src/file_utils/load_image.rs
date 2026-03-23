use dioxus::prelude::*;
use base64::{engine::general_purpose, Engine as _};
use rfd::AsyncFileDialog;
use std::fs;

#[allow(unused)]
pub fn load_image(mut image_src: Signal<String>) -> impl FnMut(MouseEvent) + 'static {
    move |_| {
        spawn(async move {
            if let Some(file) = AsyncFileDialog::new()
                .add_filter("Images", &["png", "jpg", "jpeg", "gif", "webp"])
                .pick_file()
                .await
            {
                if let Ok(bytes) = fs::read(&file.path()) {
                    let base64_data = general_purpose::STANDARD.encode(&bytes);

                    let mime_type = match file.path().extension().and_then(|e| e.to_str()) {
                        Some("png") => "image/png",
                        Some("jpg") | Some("jpeg") => "image/jpeg",
                        Some("gif") => "image/gif",
                        Some("webp") => "image/webp",
                        _ => "image/png",
                    };

                    let data_url = format!("data:{};base64,{}", mime_type, base64_data);
                    image_src.set(data_url);
                }
            }
        });
    }
}