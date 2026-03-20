use std::fs;

use quick_xml::{
    Reader,
    events::{BytesStart, Event},
    name::QName,
};

use crate::{
    error::{Error, Result},
    models::song::{AutomixVariant, Poi, Song},
};

pub fn parse_database(path: &str) -> Result<Vec<Song>> {
    let xml = fs::read_to_string(path)?;
    parse_database_str(&xml)
}

pub fn parse_database_str(xml: &str) -> Result<Vec<Song>> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut songs = Vec::new();
    let mut current_song: Option<Song> = None;

    loop {
        match reader.read_event()? {
            Event::Start(ref e) if e.name() == QName(b"Song") => {
                current_song = Some(parse_song_start(e)?);
            }

            Event::Empty(ref e) if e.name() == QName(b"Tags") => {
                if let Some(song) = current_song.as_mut() {
                    parse_tags(song, e)?;
                }
            }

            Event::Empty(ref e) if e.name() == QName(b"Infos") => {
                if let Some(song) = current_song.as_mut() {
                    parse_infos(song, e)?;
                }
            }

            Event::Start(ref e) if e.name() == QName(b"Comment") => {
                if let Some(song) = current_song.as_mut() {
                    song.comment = Some(read_comment_text(&mut reader)?);
                }
            }

            Event::Empty(ref e) if e.name() == QName(b"Scan") => {
                if let Some(song) = current_song.as_mut() {
                    parse_scan(song, e)?;
                }
            }

            Event::Empty(ref e) if e.name() == QName(b"Poi") => {
                if let Some(song) = current_song.as_mut() {
                    match parse_poi(e) {
                        Ok(poi) => song.pois.push(poi),
                        Err(err) => {
                            println!(
                                "Skipped poi for {}, err: {:#?}",
                                song.path.as_deref().unwrap_or("EMPTY PATH"),
                                err
                            );
                            continue;
                        }
                    };
                }
            }

            Event::End(ref e) if e.name() == QName(b"Song") => {
                if let Some(song) = current_song.take() {
                    songs.push(song);
                }
            }

            Event::Eof => break,
            _ => {}
        }
    }

    Ok(songs)
}

fn parse_song_start(e: &BytesStart<'_>) -> Result<Song> {
    let mut song = Song::default();

    for attr in e.attributes() {
        let attr = attr?;
        let key = attr.key.as_ref();
        let value = decode_attr_value(&attr)?;

        match key {
            b"FilePath" => song.path = value,
            b"FileSize" => {
                if let Some(v) = value {
                    song.file_size_bytes = Some(v.parse()?);
                }
            }
            b"Flag" => song.flag = parse_opt_num(value)?,
            _ => {}
        }
    }

    Ok(song)
}

fn parse_tags(song: &mut Song, e: &BytesStart<'_>) -> Result<()> {
    for attr in e.attributes() {
        let attr = attr?;
        let key = attr.key.as_ref();
        let value = decode_attr_value(&attr)?;

        match key {
            b"Author" => song.author = value,
            b"Title" => song.title = value,
            b"Genre" => song.genre = value,
            b"Album" => song.album = value,
            b"Composer" => song.composer = value,
            b"Label" => song.label = value,
            b"Remix" => song.remix = value,
            b"Remixer" => song.remixer = value,
            b"TrackNumber" => song.track_number = parse_opt_num(value)?,
            b"Grouping" => song.grouping = value,
            b"Year" => match parse_opt_num(value) {
                Ok(v) => song.year = v,
                Err(e) => {
                    eprintln!("Invalid Year value: {e}");
                    song.year = None;
                }
            },
            b"Stars" => song.stars = parse_opt_num(value)?,
            b"User1" => song.user1 = value,
            b"User2" => song.user2 = value,
            _ => {}
        }
    }

    Ok(())
}

fn parse_infos(song: &mut Song, e: &BytesStart<'_>) -> Result<()> {
    for attr in e.attributes() {
        let attr = attr?;
        let key = attr.key.as_ref();
        let value = decode_attr_value(&attr)?;

        match key {
            b"SongLength" => song.song_length_seconds = parse_opt_num(value)?,
            b"LastModified" => song.last_modified = parse_opt_num(value)?,
            b"FirstSeen" => song.first_seen = parse_opt_num(value)?,
            b"Bitrate" => song.bitrate = parse_opt_num(value)?,
            b"UserColor" => song.user_color = parse_opt_num(value)?,
            b"Cover" => song.cover = parse_opt_num(value)?,
            _ => {}
        }
    }

    Ok(())
}

fn read_comment_text(reader: &mut Reader<&[u8]>) -> Result<String> {
    let mut out = String::new();

    loop {
        match reader.read_event()? {
            Event::Text(t) => out.push_str(&t.decode()?),
            Event::CData(t) => out.push_str(&t.decode()?),
            Event::End(e) if e.name() == QName(b"Comment") => break,
            Event::Eof => return Err(Error::MalformedXml("unexpected EOF inside Comment")),
            _ => {}
        }
    }

    Ok(out)
}

fn parse_scan(song: &mut Song, e: &BytesStart<'_>) -> Result<()> {
    let mut scan = song.scan_data.take().unwrap_or_default();

    for attr in e.attributes() {
        let attr = attr?;
        let key = attr.key.as_ref();
        let value = decode_attr_value(&attr)?;

        match key {
            b"Version" => scan.version = parse_opt_num(value)?,
            b"Bpm" => scan.bpm = parse_opt_num(value)?,
            b"AltBpm" => scan.alt_bpm = parse_opt_num(value)?,
            b"Volume" => scan.volume = parse_opt_num(value)?,
            b"Key" => scan.key = value,
            b"Flag" => scan.flag = parse_opt_num(value)?,
            _ => {}
        }
    }

    song.scan_data = Some(scan);
    Ok(())
}

fn parse_poi(e: &BytesStart<'_>) -> Result<Poi> {
    let mut poi_type: Option<String> = None;
    // generic
    let mut pos: Option<f32> = None;
    let mut name: Option<String> = None;
    let mut num: Option<i16> = None;
    let mut color: Option<u32> = None;
    // action
    let mut action: Option<String> = None;
    // variable bpm
    let mut bpm: Option<f64> = None;
    // automix
    let mut point: Option<String> = None;
    // loops
    let mut size: Option<u8> = None;
    let mut slot: Option<u16> = None;
    let mut auto_trigger: Option<bool> = None;

    for attr in e.attributes() {
        let attr = attr?;
        let key = attr.key.as_ref();
        let value = decode_attr_value(&attr)?;

        match key {
            b"Type" => poi_type = value,
            b"Pos" => pos = parse_opt_num(value)?,
            b"Name" => name = value,
            b"Num" => num = parse_opt_num(value)?,
            b"Color" => color = parse_opt_num(value)?,
            b"Action" => action = value,
            b"Bpm" => bpm = parse_opt_num(value)?,
            b"Point" => point = value,
            b"Size" => size = parse_opt_num(value)?,
            b"Slot" => slot = parse_opt_num(value)?,
            b"AutoTrigger" => auto_trigger = parse_opt_bool(value)?,
            _ => {}
        }
    }

    let poi = match poi_type.as_deref() {
        Some("beatgrid") => {
            let pos = require(pos, "beatgrid", "Pos")?;
            Poi::Beatgrid { pos, bpm }
        }

        Some("remix") => {
            let pos = require(pos, "remix", "Pos")?;
            Poi::Remix {
                pos,
                num,
                name,
                color,
            }
        }

        Some("loop") => {
            let pos = require(pos, "loop", "Pos")?;
            let size = require(size, "loop", "Size")?;
            Poi::Loop {
                pos,
                slot,
                size,
                auto_trigger: auto_trigger.unwrap_or(false),
            }
        }

        Some("action") => {
            let pos = require(pos, "action", "Pos")?;
            Poi::Action {
                pos,
                num,
                name,
                action,
            }
        }

        Some("automix") => {
            let pos = pos.unwrap_or(0.0); // default in vdj is 0.0 for some reason
            let point = require(point.as_deref(), "automix", "Point")?;
            let variant = parse_automix_point(Some(point))?;
            Poi::Automix { pos, variant }
        }

        _ => {
            let pos = require(pos, "cue", "Pos")?;
            Poi::Cue {
                pos,
                num,
                name,
                color,
                load: poi_type.as_deref() == Some("load"),
            }
        }
    };

    Ok(poi)
}

fn parse_automix_point(value: Option<&str>) -> Result<AutomixVariant> {
    match value {
        Some("cutStart") => Ok(AutomixVariant::CutStart),
        Some("fadeStart") => Ok(AutomixVariant::FadeStart),
        Some("realStart") => Ok(AutomixVariant::RealStart),
        Some("tempoStart") => Ok(AutomixVariant::TempoStart),
        Some("cutEnd") => Ok(AutomixVariant::CutEnd),
        Some("fadeEnd") => Ok(AutomixVariant::FadeEnd),
        Some("realEnd") => Ok(AutomixVariant::RealEnd),
        Some("tempoEnd") => Ok(AutomixVariant::TempoEnd),
        _ => Err(Error::InvalidPoi("automix")),
    }
}

fn require<T>(v: Option<T>, poi: &'static str, field: &'static str) -> Result<T> {
    v.ok_or(Error::MissingPoiField(poi, field))
}

fn decode_attr_value(
    attr: &quick_xml::events::attributes::Attribute<'_>,
) -> Result<Option<String>> {
    let raw = std::str::from_utf8(&attr.value)?;

    match quick_xml::escape::unescape(raw) {
        Ok(v) => Ok(Some(v.into_owned())),
        Err(_) => {
            // in case file was generated falsly
            let sanitized = sanitize_ampersands(raw);

            let v = quick_xml::escape::unescape(&sanitized)
                .map_err(|e| Error::Xml(quick_xml::Error::Escape(e)))?
                .into_owned();

            Ok(Some(v))
        }
    }
}

fn sanitize_ampersands(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'&' {
            if input[i..].starts_with("&amp;")
                || input[i..].starts_with("&lt;")
                || input[i..].starts_with("&gt;")
                || input[i..].starts_with("&quot;")
                || input[i..].starts_with("&apos;")
                || input[i..].starts_with("&#")
            {
                out.push('&');
            } else {
                out.push_str("&amp;");
            }
            i += 1;
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }

    out
}

fn parse_opt_bool(value: Option<String>) -> Result<Option<bool>> {
    match value.as_deref() {
        None => Ok(None),
        Some("yes") | Some("1") | Some("true") | Some("TRUE") => Ok(Some(true)),
        _ => Ok(Some(false)),
    }
}

fn parse_opt_num<T>(value: Option<String>) -> Result<Option<T>>
where
    T: std::str::FromStr,
    Error: From<<T as std::str::FromStr>::Err>,
{
    match value {
        None => Ok(None),
        Some(v) if v.trim().is_empty() => Ok(None),
        Some(v) => Ok(Some(v.parse()?)),
    }
}
