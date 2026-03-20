use std::{
    fs::File,
    io::{Cursor, Write},
};

use quick_xml::{
    Writer,
    events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event},
};

use crate::{
    error::Result,
    models::song::{Poi, Song},
};

pub fn write_database(path: &str, songs: &[Song]) -> Result<()> {
    let xml = database_to_string(songs)?;
    let mut file = File::create(path)?;
    file.write_all(xml.as_bytes())?;
    Ok(())
}

pub fn database_to_string(songs: &[Song]) -> Result<String> {
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 1);

    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
    writer.write_event(Event::Text(BytesText::new("\n")))?;

    let mut root = BytesStart::new("VirtualDJ_Database");
    root.push_attribute(("Version", "8.1"));
    writer.write_event(Event::Start(root))?;

    for song in songs {
        write_song(&mut writer, song)?;
    }

    writer.write_event(Event::End(BytesEnd::new("VirtualDJ_Database")))?;

    let bytes = writer.into_inner().into_inner();
    let xml = String::from_utf8(bytes)?;
    Ok(xml.replace('\n', "\r\n"))
}

fn write_song<W: Write>(writer: &mut Writer<W>, song: &Song) -> Result<()> {
    let mut elem = BytesStart::new("Song");

    if let Some(v) = song.path.as_deref() {
        elem.push_attribute(("FilePath", v));
    }

    if let Some(v) = song.file_size_bytes {
        let s = v.to_string();
        elem.push_attribute(("FileSize", s.as_str()));
    }

    writer.write_event(Event::Start(elem))?;

    write_tags(writer, song)?;
    write_infos(writer, song)?;
    write_comment(writer, song)?;
    write_scan(writer, song)?;
    write_pois(writer, song)?;

    writer.write_event(Event::End(BytesEnd::new("Song")))?;
    Ok(())
}

fn write_tags<W: Write>(writer: &mut Writer<W>, song: &Song) -> Result<()> {
    let mut elem = BytesStart::new("Tags");

    if let Some(v) = song.author.as_deref() {
        elem.push_attribute(("Author", v));
    }
    if let Some(v) = song.title.as_deref() {
        elem.push_attribute(("Title", v));
    }
    if let Some(v) = song.genre.as_deref() {
        elem.push_attribute(("Genre", v));
    }
    if let Some(v) = song.album.as_deref() {
        elem.push_attribute(("Album", v));
    }
    if let Some(v) = song.composer.as_deref() {
        elem.push_attribute(("Composer", v));
    }
    if let Some(v) = song.label.as_deref() {
        elem.push_attribute(("Label", v));
    }
    if let Some(v) = song.remix.as_deref() {
        elem.push_attribute(("Remix", v));
    }
    if let Some(v) = song.remixer.as_deref() {
        elem.push_attribute(("Remixer", v));
    }
    if let Some(v) = song.grouping.as_deref() {
        elem.push_attribute(("Grouping", v));
    }
    if let Some(v) = song.user1.as_deref() {
        elem.push_attribute(("User1", v));
    }
    if let Some(v) = song.user2.as_deref() {
        elem.push_attribute(("User2", v));
    }

    if let Some(v) = song.track_number {
        let s = v.to_string();
        elem.push_attribute(("TrackNumber", s.as_str()));
    }

    if let Some(v) = song.year {
        let s = v.to_string();
        elem.push_attribute(("Year", s.as_str()));
    }

    if let Some(v) = song.stars {
        let s = v.to_string();
        elem.push_attribute(("Stars", s.as_str()));
    }

    if let Some(v) = song.flag {
        let s = v.to_string();
        elem.push_attribute(("Flag", s.as_str()));
    }

    writer.write_event(Event::Empty(elem))?;
    Ok(())
}

fn write_infos<W: Write>(writer: &mut Writer<W>, song: &Song) -> Result<()> {
    let mut elem = BytesStart::new("Infos");

    if let Some(v) = song.song_length_seconds {
        let s = v.to_string();
        elem.push_attribute(("SongLength", s.as_str()));
    }

    if let Some(v) = song.last_modified {
        let s = v.to_string();
        elem.push_attribute(("LastModified", s.as_str()));
    }

    if let Some(v) = song.first_seen {
        let s = v.to_string();
        elem.push_attribute(("FirstSeen", s.as_str()));
    }

    if let Some(v) = song.bitrate {
        let s = v.to_string();
        elem.push_attribute(("Bitrate", s.as_str()));
    }

    if let Some(v) = song.user_color {
        let s = v.to_string();
        elem.push_attribute(("UserColor", s.as_str()));
    }

    if let Some(v) = song.cover {
        let s = v.to_string();
        elem.push_attribute(("Cover", s.as_str()));
    }

    writer.write_event(Event::Empty(elem))?;
    Ok(())
}

fn write_comment<W: Write>(writer: &mut Writer<W>, song: &Song) -> Result<()> {
    if let Some(comment) = song.comment.as_deref() {
        writer.write_event(Event::Start(BytesStart::new("Comment")))?;
        writer.write_event(Event::Text(BytesText::new(comment)))?;
        writer.write_event(Event::End(BytesEnd::new("Comment")))?;
    }
    Ok(())
}

fn write_scan<W: Write>(writer: &mut Writer<W>, song: &Song) -> Result<()> {
    let Some(scan) = song.scan_data.as_ref() else {
        return Ok(());
    };

    let mut elem = BytesStart::new("Scan");

    if let Some(v) = scan.version {
        let s = v.to_string();
        elem.push_attribute(("Version", s.as_str()));
    }

    if let Some(v) = scan.bpm {
        let s = format!("{:.6}", v);
        elem.push_attribute(("Bpm", s.as_str()));
    }

    if let Some(v) = scan.alt_bpm {
        let s = format!("{:.6}", v);
        elem.push_attribute(("AltBpm", s.as_str()));
    }

    if let Some(v) = scan.volume {
        let s = v.to_string();
        elem.push_attribute(("Volume", s.as_str()));
    }

    if let Some(v) = scan.key.as_deref() {
        elem.push_attribute(("Key", v));
    }

    if let Some(v) = scan.flag {
        let s = v.to_string();
        elem.push_attribute(("Flag", s.as_str()));
    }

    writer.write_event(Event::Empty(elem))?;
    Ok(())
}

fn write_pois<W: Write>(writer: &mut Writer<W>, song: &Song) -> Result<()> {
    for poi in &song.pois {
        let mut elem = BytesStart::new("Poi");

        match poi {
            Poi::Beatgrid { pos, bpm } => {
                let pos_s = pos.to_string();
                elem.push_attribute(("Pos", pos_s.as_str()));
                elem.push_attribute(("Type", "beatgrid"));

                if let Some(v) = bpm {
                    let s = v.to_string();
                    elem.push_attribute(("Bpm", s.as_str()));
                }
            }

            Poi::Remix {
                pos,
                num,
                name,
                color,
            } => {
                let pos_s = pos.to_string();
                elem.push_attribute(("Pos", pos_s.as_str()));
                elem.push_attribute(("Type", "remix"));

                if let Some(v) = num {
                    let s = v.to_string();
                    elem.push_attribute(("Num", s.as_str()));
                }

                if let Some(v) = name.as_deref() {
                    elem.push_attribute(("Name", v));
                }

                if let Some(v) = color {
                    let s = v.to_string();
                    elem.push_attribute(("Color", s.as_str()));
                }
            }

            Poi::Loop {
                pos,
                slot,
                size,
                auto_trigger,
            } => {
                let pos_s = pos.to_string();
                elem.push_attribute(("Pos", pos_s.as_str()));
                elem.push_attribute(("Type", "loop"));

                if let Some(v) = slot {
                    let s = v.to_string();
                    elem.push_attribute(("Slot", s.as_str()));
                }

                let size = size.to_string();
                elem.push_attribute(("Size", size.as_str()));

                if *auto_trigger {
                    elem.push_attribute(("AutoTrigger", "1"));
                }
            }

            Poi::Action {
                pos,
                num,
                name,
                action,
            } => {
                let pos_s = pos.to_string();
                elem.push_attribute(("Pos", pos_s.as_str()));
                elem.push_attribute(("Type", "action"));

                if let Some(v) = action.as_deref() {
                    elem.push_attribute(("Action", v));
                }

                if let Some(v) = num {
                    let s = v.to_string();
                    elem.push_attribute(("Num", s.as_str()));
                }

                if let Some(v) = name.as_deref() {
                    elem.push_attribute(("Name", v));
                }
            }

            Poi::Automix { pos, variant } => {
                let pos_s = pos.to_string();
                elem.push_attribute(("Pos", pos_s.as_str()));
                elem.push_attribute(("Type", "automix"));
                elem.push_attribute(("Point", variant.as_str()));
            }

            Poi::Cue {
                pos,
                num,
                name,
                color,
                load,
            } => {
                let pos_s = pos.to_string();
                elem.push_attribute(("Pos", pos_s.as_str()));

                if *load {
                    elem.push_attribute(("Type", "load"));
                } else {
                    elem.push_attribute(("Type", "cue"));
                }

                if let Some(v) = num {
                    let s = v.to_string();
                    elem.push_attribute(("Num", s.as_str()));
                }

                if let Some(v) = name.as_deref() {
                    elem.push_attribute(("Name", v));
                }

                if let Some(v) = color {
                    let s = v.to_string();
                    elem.push_attribute(("Color", s.as_str()));
                }
            }
        }

        writer.write_event(Event::Empty(elem))?;
    }

    Ok(())
}
