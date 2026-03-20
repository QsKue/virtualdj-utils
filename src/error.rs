use std::{
    fmt, io,
    num::{ParseFloatError, ParseIntError},
    string::FromUtf8Error,
};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Xml(quick_xml::Error),
    Attr(quick_xml::events::attributes::AttrError),
    Encoding(quick_xml::encoding::EncodingError),
    Utf8(std::str::Utf8Error),
    FromUtf8(FromUtf8Error),
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
    InvalidPoi(&'static str),
    MissingPoiField(&'static str, &'static str),
    MissingSongPath,
    InvalidBool(String),
    InvalidKey(String),
    MalformedXml(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "io error: {e}"),
            Error::Xml(e) => write!(f, "xml error: {e}"),
            Error::Attr(e) => write!(f, "xml attribute error: {e}"),
            Error::Encoding(e) => write!(f, "xml encoding error: {e}"),
            Error::Utf8(e) => write!(f, "utf8 error: {e}"),
            Error::FromUtf8(e) => write!(f, "from utf8 error: {e}"),
            Error::ParseInt(e) => write!(f, "integer parse error: {e}"),
            Error::ParseFloat(e) => write!(f, "float parse error: {e}"),
            Error::InvalidPoi(msg) => write!(f, "invalid poi: {msg}"),
            Error::MissingPoiField(poi, msg) => write!(f, "{poi} is missing field: {msg}"),
            Error::MissingSongPath => write!(f, "missing required Song FilePath attribute"),
            Error::InvalidBool(v) => write!(f, "invalid bool value: {v}"),
            Error::InvalidKey(v) => write!(f, "invalid key value: {v}"),
            Error::MalformedXml(msg) => write!(f, "malformed xml: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(value: quick_xml::Error) -> Self {
        Self::Xml(value)
    }
}

impl From<quick_xml::events::attributes::AttrError> for Error {
    fn from(value: quick_xml::events::attributes::AttrError) -> Self {
        Self::Attr(value)
    }
}

impl From<quick_xml::encoding::EncodingError> for Error {
    fn from(value: quick_xml::encoding::EncodingError) -> Self {
        Self::Encoding(value)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::Utf8(value)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Self::FromUtf8(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl From<ParseFloatError> for Error {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloat(value)
    }
}
