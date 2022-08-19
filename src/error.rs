
#[derive(Debug)]
pub enum Error {
    HttpError(u16, Option<String>),
    SerdeError(String),
    ReqwestError(String)
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        let status = match err.status() {
            None => return Error::ReqwestError(format!("{:?}", err)),
            Some(status) => status.as_u16()
        };

        Error::HttpError(status, None)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeError(format!("{:?}", err))
    }
}
