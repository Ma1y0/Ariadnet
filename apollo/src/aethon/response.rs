use super::{headers::Headers, status::Status, Error};
use std::{
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

impl FromStr for Response {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn test_parse_str() {
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
}
