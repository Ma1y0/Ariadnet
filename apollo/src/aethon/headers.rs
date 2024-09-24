use std::{
    collections::BTreeMap,
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[derive(Debug, PartialEq)]
pub struct Headers(BTreeMap<Box<str>, Box<str>>);

impl Headers {
    pub fn new() -> Self {
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
