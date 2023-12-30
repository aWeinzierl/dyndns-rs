use preferences::PreferencesError;

#[derive(Debug)]
pub enum Error {
    PreferencesError(preferences::PreferencesError),
    ReqwestError(reqwest::Error),
    ResolverError(String),
    AuthenticationError(String),
    GoDaddyError(crate::godaddy::Error),
    YDns(crate::godaddy::Error),
}

impl From<preferences::PreferencesError> for Error {
    fn from(error: PreferencesError) -> Self {
        Error::PreferencesError(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}

impl From<(Error, usize)> for Error {
    fn from(error: (Error, usize)) -> Self {
        error.0
    }
}

impl From<crate::godaddy::Error> for Error {
    fn from(error: crate::godaddy::Error) -> Self {
        Error::GoDaddyError(error)
    }
}
