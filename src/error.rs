
#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ReqwestError(reqwest::Error),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
    UrlParseError(url::ParseError),
    SerdeJsonError(serde_json::Error),
    ToStrError(reqwest::header::ToStrError),
    Nan,
    NotAString,
    Unimplemented,
    RaylibShaderError,
    InvalidKey,
    InvalidColor,
}

impl From<std::io::Error> for Error {
    fn from(o: std::io::Error) -> Self {
        Self::IOError(o)
    }
}

impl From<reqwest::Error> for Error {
    fn from(o: reqwest::Error) -> Self {
        Self::ReqwestError(o)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(o: reqwest::header::InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(o)
    }
}

impl From<url::ParseError> for Error {
    fn from(o: url::ParseError) -> Self {
        Self::UrlParseError(o)
    }
}

impl From<serde_json::Error> for Error {
    fn from(o: serde_json::Error) -> Self {
        Self::SerdeJsonError(o)
    }
}

impl From<reqwest::header::ToStrError> for Error {
    fn from(o: reqwest::header::ToStrError) -> Self {
        Self::ToStrError(o)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &'_ mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
