pub enum Error {
    ResourceNotFound,
    NoContent,
    UnavailableAssetName,
    Unexpected(String)
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        let status = match err.status() {
            None => return Error::Unexpected(format!("{:?}", err)),
            Some(status) => status.as_u16()
        };

        match status {
            404 => Error::ResourceNotFound,
            204 => Error::NoContent,
            422 => Error::UnavailableAssetName,
            _ => Error::Unexpected(format!("{:?}", err))
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Unexpected(format!("{:?}", err))
    }
}
