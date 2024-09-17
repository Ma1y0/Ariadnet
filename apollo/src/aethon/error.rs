use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    WrongMethod,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongMethod => write!(f, "The request METHOD is invalid"),
        }
    }
}
