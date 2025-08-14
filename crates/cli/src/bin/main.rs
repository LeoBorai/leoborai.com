use std::env::current_dir;

use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;

use libcli::notes_index::NotesIndex;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cli_opts = Cli::parse();

    cli_opts.exec().await.expect("Command Failed");
}

#[derive(Debug, Parser)]
#[command(next_line_help = true)]
#[command(name = "website", author, version, about)]
pub enum Cli {
    /// Creates the Notes Entry Index
    Notes,
}

impl Cli {
    pub async fn exec(self) -> Result<()> {
        match self {
            Self::Notes => {
                let index_path = current_dir()?.join("static").join("notes");
                let index = NotesIndex::new()?;
                index.save_to_file(index_path)?;
            }
        }

        Ok(())
    }
}
