use std::fs;

use crate::{
    error::Result,
    models::m3u::{M3u, M3uSong},
};

type SongMetadata = (Option<String>, Option<String>, Option<f32>);

fn extract_tag_value(input: &str, tag: &str) -> Option<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);

    let start = input.find(&open)? + open.len();
    let end = input[start..].find(&close)? + start;

    Some(input[start..end].to_string())
}

fn parse_extvdj_line(line: &str) -> SongMetadata {
    let payload = line.strip_prefix("#EXTVDJ:").unwrap_or("");

    let artist = extract_tag_value(payload, "artist");
    let title = extract_tag_value(payload, "title");
    let songlength = extract_tag_value(payload, "songlength").and_then(|s| s.parse::<f32>().ok());

    (artist, title, songlength)
}

pub fn parse_m3u(path: &str) -> Result<M3u> {
    let xml = fs::read_to_string(path)?;
    Ok(parse_m3u_str(&xml))
}

pub fn parse_m3u_str(input: &str) -> M3u {
    let mut songs = Vec::new();
    let mut pending: Option<SongMetadata> = None;

    for raw_line in input.lines() {
        let line = raw_line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("#EXTVDJ:") {
            pending = Some(parse_extvdj_line(line));
            continue;
        }

        if line.starts_with('#') {
            continue;
        }

        // This is a filepath line
        let meta = pending.take().unwrap_or_default();

        songs.push(M3uSong {
            path: line.to_string(),
            artist: meta.0,
            title: meta.1,
            songlength: meta.2,
        });
    }

    M3u { songs }
}
