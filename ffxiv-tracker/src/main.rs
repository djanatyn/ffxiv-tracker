mod db;
mod job;
mod profile;

use std::path::PathBuf;

use db::TrackerDatabase;
use profile::Profile;

use clap::{Parser, Subcommand};
use tracing::{event, info_span, instrument, Level};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Download/update files for running tests.
    Snapshot {
        user_id: u64,
        #[arg(default_value = "./ffxiv-tracker.sqlite")]
        database_path: PathBuf,
    },
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    tracing_subscriber::fmt::init();

    match cli.command {
        Command::Snapshot {
            user_id,
            database_path,
        } => snapshot(user_id, database_path),
    }
}

#[instrument]
fn snapshot(user_id: u64, database_path: PathBuf) -> Result<(), String> {
    let database = info_span!("db").in_scope(|| {
        let db = TrackerDatabase {
            path: database_path,
        };
        event!(Level::INFO, "initializing database");
        db.init()?;
        Ok::<TrackerDatabase, String>(db)
    })?;
    let profile = info_span!("fetch").in_scope(|| {
        event!(Level::INFO, "downloading profile");
        Profile::get(user_id)
    })?;

    database.snapshot(profile)
}
