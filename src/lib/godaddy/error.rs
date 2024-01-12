pub enum Error {
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    NotImplemented(&'static str),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Reqwest(e) => e.fmt(f),
            Error::SerdeJson(e) => e.fmt(f),
            Error::NotImplemented(e) => e.fmt(f),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Reqwest(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::SerdeJson(err)
    }
}

impl From<&'static str> for Error {
    fn from(err: &'static str) -> Error {
        Error::NotImplemented(err)
    }
}
