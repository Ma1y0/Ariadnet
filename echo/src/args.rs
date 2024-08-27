use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Runs the server
    Serve,
    /// Adds value to the kv store
    Add { key: String, value: String },
    /// Prints records
    Print {
        /// How many
        #[arg(short)]
        n: Option<usize>,
        /// Print records as JSON
        #[arg(long)]
        json: bool,
    },
}
