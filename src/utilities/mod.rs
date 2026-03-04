#[allow(unused)]
pub fn format_music_title(title: &str, max_len: usize) -> String {
    if title.chars().count() <= max_len {
        title.to_string()
    } else {
        format!("{}...", title.chars().take(max_len).collect::<String>())
    }
}