use std::fmt::Display;

#[derive(Debug)]
pub enum HTMLError {
    UnexpectedEndOfInput,
    InvalidSyntax,
    InvalidTag,
    Other(String),
}

impl Display for HTMLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEndOfInput => write!(f, "Unexpected End Of Input"),
            Self::InvalidSyntax => write!(f, "Invalid Syntax"),
            Self::InvalidTag => write!(f, "Invalid Tag Name"),
            Self::Other(s) => write!(f, "{}", s),
        }
    }
}
