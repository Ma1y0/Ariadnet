use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    WrongMethod,
    WrongStatus,
    ParseError(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongMethod => write!(f, "The request METHOD is invalid"),
            Self::WrongStatus => write!(f, "The response STATUS is invalid"),
            Self::ParseError(e) => write!(f, "Failed to parse the Aethon packet: {}", e),
        }
    }
}
