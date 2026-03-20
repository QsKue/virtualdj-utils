use std::{fs::File, io::Write};

use crate::{error::Result, models::m3u::M3u};

pub fn write_m3u(path: &str, m3u: &M3u) -> Result<()> {
    let m3u = m3u_to_string(m3u);
    let mut file = File::create(path)?;
    file.write_all(m3u.as_bytes())?;
    Ok(())
}

pub fn m3u_to_string(m3u: &M3u) -> String {
    let mut out = String::new();

    for song in m3u.songs.iter() {
        if song.artist.is_some() || song.title.is_some() || song.songlength.is_some() {
            out.push_str("#EXTVDJ:");

            if let Some(artist) = &song.artist {
                out.push_str("<artist>");
                out.push_str(artist);
                out.push_str("</artist>");
            }

            if let Some(title) = &song.title {
                out.push_str("<title>");
                out.push_str(title);
                out.push_str("</title>");
            }

            if let Some(songlength) = song.songlength {
                out.push_str("<songlength>");
                out.push_str(&songlength.to_string());
                out.push_str("</songlength>");
            }

            out.push('\n');
        }

        out.push_str(&song.path);
        out.push('\n');
    }

    out
}
