use super::{method::Method, Error};
use std::{
    collections::BTreeMap,
    fmt::Display,
    iter::Peekable,
    ops::{Deref, DerefMut},
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
            .map_err(|_| Error::ParseError("Version ins't a u8"))?;
        let method: Method = Self::consume_string(&mut buffer, ' ')
            .parse()
            .map_err(|_| Error::ParseError("Invalid method"))?;
        let path: String = Self::consume_string(&mut buffer, '\n');
        let headers = Self::parser_headers(&mut buffer)?;
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

    fn consume_headers(buffer: &mut Peekable<Chars>) -> String {
        let mut result = String::new();
        let mut last_char_was_newline = false;

        while let Some(&c) = buffer.peek() {
            if c == '\n' && last_char_was_newline {
                buffer.next(); // consume the \n
                break;
            }

            result.push(buffer.next().unwrap());
            last_char_was_newline = c == '\n';
        }

        result
    }

    fn parser_headers(buffer: &mut Peekable<Chars>) -> Result<Headers, Error> {
        let mut headers = Headers::new();
        let buffer = Self::consume_headers(buffer);

        for line in buffer.lines() {
            if line.is_empty() {
                break;
            }

            let mut kv = line.splitn(2, ':');
            let key = kv.next().ok_or(Error::ParseError("Invalid headers key"))?;
            let value = kv
                .next()
                .ok_or(Error::ParseError("Invalid headers value"))?;

            if key.trim().is_empty() {
                break;
            }

            let value = value.trim();
            if value.is_empty() {
                return Err(Error::ParseError("Headers value can't be empty"));
            }

            headers.insert(key.into(), value.into());
        }

        Ok(headers)
    }
}

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

#[derive(Debug, PartialEq)]
pub struct Headers(BTreeMap<Box<str>, Box<str>>);

impl Headers {
    fn new() -> Self {
        Headers(BTreeMap::new())
    }
}

impl Deref for Headers {
    type Target = BTreeMap<Box<str>, Box<str>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Headers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in self.iter() {
            writeln!(f, "{}: {}", k, v)?;
        }

        Ok(())
    }
}

impl From<BTreeMap<Box<str>, Box<str>>> for Headers {
    fn from(map: BTreeMap<Box<str>, Box<str>>) -> Self {
        Headers(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_request() {
        let s = "1 GET /\na: hello\nc: b\n\nHello World";
        let req: Request = s.parse().unwrap();

        let mut expected_headers: Headers = Headers::new();
        expected_headers.insert("a".into(), "hello".into());
        expected_headers.insert("c".into(), "b".into());

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
        let expected = Request::new(1, Method::GET, "/", Headers::new(), "");

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
