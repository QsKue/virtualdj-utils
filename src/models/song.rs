
#[derive(Debug, Clone, Default)]
pub struct Song {
    pub path: Option<String>,
    pub file_size_bytes: Option<u64>,
    pub flag: Option<u32>,

    pub author: Option<String>,
    pub title: Option<String>,
    pub genre: Option<String>,
    pub album: Option<String>,
    pub composer: Option<String>,
    pub label: Option<String>,
    pub remix: Option<String>,
    pub remixer: Option<String>,
    pub track_number: Option<u32>,
    pub grouping: Option<String>,
    pub year: Option<u16>,
    pub stars: Option<u8>,
    pub user1: Option<String>,
    pub user2: Option<String>,

    pub song_length_seconds: Option<f32>,
    pub last_modified: Option<u64>,
    pub first_seen: Option<u64>,
    pub bitrate: Option<u32>,
    pub user_color: Option<u32>,
    pub cover: Option<u16>,

    pub comment: Option<String>,

    pub scan: Option<ScanData>,

    pub pois: Vec<Poi>,
}

#[derive(Debug, Clone, Default)]
pub struct ScanData {
    pub version: Option<u16>,
    pub bpm: Option<f64>,
    pub alt_bpm: Option<f64>,
    pub volume: Option<f32>,
    pub key: Option<String>,
    pub flag: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum Poi {
    Beatgrid {
        pos: f32,
        bpm: Option<f64>,
    },
    Cue {
        pos: f32,
        num: Option<i16>,
        name: Option<String>,
        color: Option<u32>,
        load: bool,
    },
    Remix {
        pos: f32,
        num: Option<i16>,
        name: Option<String>,
        color: Option<u32>,
    },
    Loop {
        pos: f32,
        slot: Option<u16>,
        size: u8,
        auto_trigger: bool,
    },
    Action {
        pos: f32,
        num: Option<i16>,
        name: Option<String>,
        action: Option<String>,
    },
    Automix {
        pos: f32,
        variant: AutomixVariant,
    },
}

#[derive(Debug, Clone)]
pub enum AutomixVariant {
    CutStart,
    FadeStart,
    RealStart,
    TempoStart,
    CutEnd,
    FadeEnd,
    RealEnd,
    TempoEnd,
}

impl AutomixVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            AutomixVariant::CutStart => "cutStart",
            AutomixVariant::FadeStart => "fadeStart",
            AutomixVariant::RealStart => "realStart",
            AutomixVariant::TempoStart => "tempoStart",
            AutomixVariant::CutEnd => "cutEnd",
            AutomixVariant::FadeEnd => "fadeEnd",
            AutomixVariant::RealEnd => "realEnd",
            AutomixVariant::TempoEnd => "tempoEnd",
        }
    }
}