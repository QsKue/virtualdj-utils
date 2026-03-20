#[derive(Debug, Clone, PartialEq)]
pub struct M3u {
    pub songs: Vec<M3uSong>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct M3uSong {
    pub path: String,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub songlength: Option<f32>,
}
