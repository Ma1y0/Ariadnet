use super::{headers::Headers, method::Method, Error};
use std::{
    fmt::Display,
    iter::Peekable,
    str::{Chars, FromStr},
};

#[derive(Debug, PartialEq)]
pub struct Request {
    version: u8,
    method: Method,
    path: String,
    headers: Headers,
    body: String,
}

impl Request {
    pub fn new(
        version: u8, method: Method, path: impl Into<String>, headers: Headers,
        body: impl Into<String>,
    ) -> Self {
        Self {
            version,
            method,
            path: path.into(),
            headers,
            body: body.into(),
        }
    }

    fn parse(s: &str) -> Result<Self, Error> {
        let mut buffer = s.chars().peekable();

        let version: u8 = Self::consume_string(&mut buffer, ' ')
            .parse()
            .map_err(|_| Error::ParseError("Version ins't an u8"))?;
        let method: Method = Self::consume_string(&mut buffer, ' ')
            .parse()
            .map_err(|_| Error::ParseError("Invalid method"))?;
        let path: String = Self::consume_string(&mut buffer, '\n');
        let headers = Headers::parser_headers(&mut buffer)?;
        let body = buffer.collect();

        Ok(Request {
            version,
            method,
            path,
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

/// Used for parsing.
impl FromStr for Request {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Request::parse(s)
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Version
        // (1-9) + 48 = ASCII representation
        write!(f, "{} ", (self.version + 48) as char)?;
        // Method
        write!(f, "{} ", self.method)?;
        // Path
        writeln!(f, "{}", self.path)?;
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
    fn test_parser_request() {
        let s = "1 GET /\na: hello\nc: b\n\nHello World";
        let req: Request = s.parse().unwrap();

        let mut expected_headers: Headers = Headers::default();
        expected_headers.insert("a", "hello");
        expected_headers.insert("c", "b");

        let expected = Request::new(1, Method::GET, "/", expected_headers, "Hello World");

        // Tests
        assert_eq!(expected, req);
    }

    #[test]
    fn test_fail_to_parse_wrong_invalid_headers() {
        let s = "1 GET /\na: hello\nc: \n\n";
        let req: Result<Request, Error> = s.parse();

        // Tests
        assert!(req.is_err());
    }

    #[test]
    fn test_parse_packet_without_headers() {
        let s = "1 GET /\n\n\n";
        let req: Request = s.parse().unwrap();
        let expected = Request::new(1, Method::GET, "/", Headers::default(), "");

        // Tests
        assert_eq!(expected, req);
    }

    #[test]
    fn test_to_string() {
        let headers: BTreeMap<Box<str>, Box<str>> =
            BTreeMap::from([("1".into(), "Hello".into()), ("2".into(), "World".into())]);
        let req = Request::new(1, Method::GET, "/", headers.into(), "");
        let s = req.to_string();
        let expected = "1 GET /\n1: Hello\n2: World\n\n";

        // Tests
        assert_eq!(expected, s);
    }
}
