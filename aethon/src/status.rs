use super::Error;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum Status {
    // 2** Success
    OK,      // 200
    Created, // 201
    // 4** Client error
    BadRequest,       // 400
    Unauthorized,     // 401
    NotFound,         // 404
    MethodNotAllowed, // 405
    ImATeapot,        // 418 The server refuses the attempt to brew coffee with a teapot.
    // 5** Server error
    InternalServerError, // 500
}

impl FromStr for Status {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            // 2**
            "200" => Ok(Self::OK),
            "201" => Ok(Self::Created),
            // 4**
            "400" => Ok(Self::BadRequest),
            "401" => Ok(Self::Unauthorized),
            "404" => Ok(Self::NotFound),
            "405" => Ok(Self::MethodNotAllowed),
            "418" => Ok(Self::ImATeapot),
            // 5**
            "500" => Ok(Self::InternalServerError),
            _ => Err(Self::Err::WrongStatus),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OK => write!(f, "200"),
            Self::Created => write!(f, "201"),
            Self::BadRequest => write!(f, "400"),
            Self::Unauthorized => write!(f, "401"),
            Self::NotFound => write!(f, "404"),
            Self::MethodNotAllowed => write!(f, "405"),
            Self::ImATeapot => write!(f, "418"),
            Self::InternalServerError => write!(f, "500"),
        }
    }
}

impl TryFrom<u16> for Status {
    type Error = Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            200 => Ok(Status::OK),
            201 => Ok(Status::Created),
            400 => Ok(Status::BadRequest),
            401 => Ok(Status::Unauthorized),
            404 => Ok(Status::NotFound),
            405 => Ok(Status::MethodNotAllowed),
            418 => Ok(Status::ImATeapot),
            500 => Ok(Status::InternalServerError),
            _ => Err(Self::Error::WrongStatus),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_status_codes() {
        // From str
        assert_eq!(Status::from_str("200"), Ok(Status::OK));
        assert_eq!(Status::from_str("201"), Ok(Status::Created));
        assert_eq!(Status::from_str("400"), Ok(Status::BadRequest));
        assert_eq!(Status::from_str("401"), Ok(Status::Unauthorized));
        assert_eq!(Status::from_str("404"), Ok(Status::NotFound));
        assert_eq!(Status::from_str("405"), Ok(Status::MethodNotAllowed));
        assert_eq!(Status::from_str("418"), Ok(Status::ImATeapot));
        assert_eq!(Status::from_str("500"), Ok(Status::InternalServerError));

        // To string
        assert_eq!(Status::OK.to_string(), "200");
        assert_eq!(Status::Created.to_string(), "201");
        assert_eq!(Status::BadRequest.to_string(), "400");
        assert_eq!(Status::Unauthorized.to_string(), "401");
        assert_eq!(Status::NotFound.to_string(), "404");
        assert_eq!(Status::MethodNotAllowed.to_string(), "405");
        assert_eq!(Status::ImATeapot.to_string(), "418");
        assert_eq!(Status::InternalServerError.to_string(), "500");

        // From number
        assert_eq!(Status::try_from(200), Ok(Status::OK));
        assert_eq!(Status::try_from(201), Ok(Status::Created));
        assert_eq!(Status::try_from(400), Ok(Status::BadRequest));
        assert_eq!(Status::try_from(401), Ok(Status::Unauthorized));
        assert_eq!(Status::try_from(404), Ok(Status::NotFound));
        assert_eq!(Status::try_from(405), Ok(Status::MethodNotAllowed));
        assert_eq!(Status::try_from(418), Ok(Status::ImATeapot));
        assert_eq!(Status::try_from(500), Ok(Status::InternalServerError));
    }

    #[test]
    fn test_invalid_status_codes() {
        assert_eq!(Status::from_str("199"), Err(Error::WrongStatus));
        assert_eq!(Status::from_str("300"), Err(Error::WrongStatus));
        assert_eq!(Status::from_str("600"), Err(Error::WrongStatus));
        assert_eq!(Status::try_from(999), Err(Error::WrongStatus));
    }

    #[test]
    fn test_whitespace_handling() {
        assert_eq!(Status::from_str(" 200 "), Ok(Status::OK));
        assert_eq!(Status::from_str("\t404\n"), Ok(Status::NotFound));
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(Status::from_str(""), Err(Error::WrongStatus));
    }

    #[test]
    fn test_non_numeric_input() {
        assert_eq!(Status::from_str("OK"), Err(Error::WrongStatus));
        assert_eq!(Status::from_str("Not Found"), Err(Error::WrongStatus));
    }
}
