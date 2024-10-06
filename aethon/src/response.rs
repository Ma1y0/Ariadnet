use super::{headers::Headers, status::Status, Error};
use std::{
    fmt::Display,
    iter::Peekable,
    str::{Chars, FromStr},
};

#[derive(Debug, PartialEq)]
pub struct Response {
    version: u8,
    status: Status,
    headers: Headers,
    body: String,
}

impl Response {
    pub fn new(version: u8, status: Status, headers: Headers, body: impl Into<String>) -> Self {
        Self {
            version,
            status,
            headers,
            body: body.into(),
        }
    }

    fn parse(s: &str) -> Result<Self, Error> {
        let mut buffer = s.chars().peekable();

        let version: u8 = Self::consume_string(&mut buffer, ' ')
            .parse()
            .map_err(|_| Error::ParseError("Version isn't an u8"))?;
        let status: Status = Self::consume_string(&mut buffer, '\n')
            .parse()
            .map_err(|_| Error::ParseError("Invalid status"))?;
        let headers: Headers = Headers::parser_headers(&mut buffer)?;
        let body: String = buffer.collect();

        Ok(Response {
            version,
            status,
            headers,
            body,
        })
    }

    fn consume_string(buffer: &mut Peekable<Chars>, to: char) -> String {
        let mut s = String::new();

        for ch in buffer {
            if ch == to {
                break;
            }

            s.push(ch);
        }

        s
    }
}

/// Used for parsing
impl FromStr for Response {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&[u8]> for Response {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let buffer = String::from_utf8_lossy(value);
        Response::parse(&buffer)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Version
        // (1-9) + 48 = ASCII representation
        write!(f, "{} ", (self.version + 48) as char)?;
        // Status
        writeln!(f, "{}", self.status)?;
        // Headers
        writeln!(f, "{}", self.headers)?;
        // Body
        write!(f, "{}", self.body)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn test_parse_tesponse() {
        let s: Result<Response, Error> = "1 200\nh: Hello\nw:World\n\nHello World".parse();
        let expected = Ok(Response::new(
            1,
            Status::OK,
            Headers::from(BTreeMap::from([
                ("h".into(), "Hello".into()),
                ("w".into(), "World".into()),
            ])),
            "Hello World",
        ));

        // Tests
        assert_eq!(expected, s);
    }

    #[test]
    fn test_parse_tesponse_u8_array() {
        let s: Result<Response, Error> =
            Response::try_from("1 200\nh: Hello\nw:World\n\nHello World".as_bytes());
        let expected = Ok(Response::new(
            1,
            Status::OK,
            Headers::from(BTreeMap::from([
                ("h".into(), "Hello".into()),
                ("w".into(), "World".into()),
            ])),
            "Hello World",
        ));

        // Tests
        assert_eq!(expected, s);
    }

    #[test]
    fn test_response_to_string() {
        let res = Response::new(
            1,
            Status::OK,
            Headers::from(BTreeMap::from([
                ("h".into(), "Hello".into()),
                ("w".into(), "World".into()),
            ])),
            "Hello World",
        );
        let expected = "1 200\nh: Hello\nw: World\n\nHello World";

        // Tests
        assert_eq!(expected, res.to_string());
    }
}
