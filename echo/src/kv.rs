use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io,
    ops::{Deref, DerefMut},
};
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[derive(Serialize, Deserialize)]
pub struct Store(HashMap<Box<str>, Box<str>>);

impl Store {
    pub async fn load() -> anyhow::Result<Store> {
        Self::read().await
    }

    pub fn in_memory() -> Self {
        Self::new()
    }

    pub fn insert(&mut self, key: &str, value: &str) -> Option<Box<str>> {
        self.0.insert(key.into(), value.into())
    }

    pub async fn insert_write(&mut self, key: &str, value: &str) -> anyhow::Result<()> {
        self.insert(key, value);
        self.write().await
    }

    // Writes Store into store.bin file
    // Slow (I think)
    pub async fn write(&self) -> anyhow::Result<()> {
        let mut file = fs::File::create("./store.bin")
            .await
            .context("Failed to open/crate store.bin")?;

        // Serialize the data
        let serialized = bincode::serialize(self).context("Failed to serialize the sore")?;

        // Writes to the file
        file.write_all(&serialized)
            .await
            .context("Failed to write data to file")?;
        // Makes sure the data were written to the file
        file.sync_all().await.context("Failed to sync the file")
    }

    fn new() -> Store {
        Store(HashMap::new())
    }

    // Reads Store from store.bin
    async fn read() -> anyhow::Result<Self> {
        match fs::File::open("./store.bin").await {
            Ok(mut file) => {
                // Reads the data
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)
                    .await
                    .context("Failed to read the store.bin file")?;

                // Deserialize the data
                bincode::deserialize(&buffer).context("Failed to deserialize data")
            }
            // File not found
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                let store = Self::new();
                store
                    .write()
                    .await
                    .context("Failed to write into store.bin")?;
                Ok(store)
            }
            Err(e) => Err(e.into()),
        }
    }
}

impl Deref for Store {
    type Target = HashMap<Box<str>, Box<str>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Store {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
