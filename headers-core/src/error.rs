#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

#[derive(Debug)]
enum Kind {
    Invalid,
    Empty,
    TooMany,
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl Error {
    fn new(kind: Kind) -> Self {
        Error {
            kind,
        }
    }

    pub fn invalid() -> Self {
        Error::new(Kind::Invalid)
    }

    pub fn empty() -> Self {
        Error::new(Kind::Empty)
    }

    pub fn too_many_values() -> Self {
        Error::new(Kind::TooMany)
    }
}

impl From<::http::header::ToStrError> for Error {
    fn from(_: ::http::header::ToStrError) -> Error {
        Error::invalid()
    }
}

