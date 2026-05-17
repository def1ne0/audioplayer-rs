#[derive(Clone, PartialEq, Eq)]
pub struct Track {
    pub cover_src: Option<String>,
    pub path: String,
    pub name: String,
}