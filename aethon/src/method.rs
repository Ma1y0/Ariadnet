use super::Error;
use core::fmt;
use std::str::FromStr;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum Method {
    /// The `GET` method requests some content.
    GET,
    /// The `POST` method submits data to the server.
    POST,
    /// The `DELETE` method requests deletion of an item.
    DELETE,
}

/// Used for parsing.
impl FromStr for Method {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "DELETE" => Ok(Self::DELETE),
            _ => Err(self::Error::WrongMethod),
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GET => write!(f, "GET"),
            Self::POST => write!(f, "POST"),
            Self::DELETE => write!(f, "DELETE"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_method() {
        let s = ["GET", "POST", "DELETE", "aaaaAAkkfe"];
        let res: Vec<Result<Method, Error>> = s.iter().map(|x| x.parse()).collect();

        // Tests
        let expected = vec![
            Ok(Method::GET),
            Ok(Method::POST),
            Ok(Method::DELETE),
            Err(Error::WrongMethod),
        ];
        assert_eq!(expected, res);
    }

    #[test]
    fn test_method_to_str() {
        let methods = [Method::GET, Method::POST, Method::DELETE];
        let res: Vec<String> = methods.iter().map(|x| x.to_string()).collect();
        let expected: Vec<String> = ["GET", "POST", "DELETE"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        // Tests
        assert_eq!(res, expected)
    }
}
