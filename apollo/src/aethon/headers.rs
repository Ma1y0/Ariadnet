use std::{
    collections::BTreeMap,
    fmt::Display,
    iter::Peekable,
    ops::{Deref, DerefMut},
    str::Chars,
};

use super::Error;

#[derive(Debug, PartialEq)]
pub struct Headers(BTreeMap<Box<str>, Box<str>>);

impl Headers {
    pub fn new() -> Self {
        Headers(BTreeMap::new())
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

    pub fn parser_headers(buffer: &mut Peekable<Chars>) -> Result<Headers, Error> {
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
