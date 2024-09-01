use anyhow::Context;
use clap::Parser;
use echo::{
    args::{Args, Commands},
    kv::{self, Store},
    server::serve,
};
use std::collections::HashMap;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut store = kv::Store::load().await?;

    match args.command {
        Commands::Add { key, value } => add_record(&mut store, &key, &value).await,
        Commands::Print { n, json } => print_records(&store, n, json),
        Commands::Serve => {
            let port = std::env::var("ECHO_PORT").unwrap_or("53".to_string());
            let socket = UdpSocket::bind(format!("127.0.0.1:{port}").as_str()).await?;

            serve(socket).await
        }
    }
}

/// Prints records
fn print_records(store: &Store, n: Option<usize>, json: bool) -> anyhow::Result<()> {
    if json {
        // Prints as JSON
        match n {
            Some(n) => {
                let store: HashMap<_, _> = store.iter().take(n).collect();
                println!(
                    "{}",
                    serde_json::to_string_pretty(&store)
                        .context("Failed to convert map to json")?
                )
            }
            None => println!(
                "{}",
                serde_json::to_string_pretty(&store).context("Failed to convert map to json")?
            ),
        }
    } else {
        // Prints n kv pairs or prints the whole map
        for (key, value) in store.iter().take(n.unwrap_or(store.len())) {
            println!("{key}: {value}")
        }
    }

    Ok(())
}

/// Adds kv record to the store
async fn add_record(store: &mut Store, key: &str, value: &str) -> anyhow::Result<()> {
    store.insert(key, value).await;
    Ok(())
}
