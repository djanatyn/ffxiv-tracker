mod job;
mod profile;

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
    Snapshot { user_id: u64 },
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    tracing_subscriber::fmt::init();

    match cli.command {
        Command::Snapshot { user_id } => snapshot(user_id),
    }
}

#[instrument]
fn snapshot(user_id: u64) -> Result<(), String> {
    let profile = info_span!("fetch").in_scope(|| {
        event!(Level::INFO, "downloading profile");
        Profile::get(user_id)
    })?;
    todo!()
}
