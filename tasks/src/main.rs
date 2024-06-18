use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use tracing::{event, info_span, instrument, Level};

const PROFILE_BASE_URL: &'static str = "https://na.finalfantasyxiv.com/lodestone/character/";
const PROFILES: &'static str = include_str!("tests/test-profiles.kdl");

#[derive(Debug, knuffel::Decode)]
struct TestProfile {
    #[knuffel(argument)]
    name: String,
    #[knuffel(property)]
    id: u64,
}

impl TestProfile {
    #[instrument(skip_all)]
    fn fetch(self, output_directory: &Path) -> Result<(), String> {
        let TestProfile { name, id } = &self;

        let profile = output_directory.join(PathBuf::from(format!("{name}_profile.html")));
        let job = output_directory.join(PathBuf::from(format!("{name}_jobs.html")));

        let profile_url = format!("{0}/{id}", PROFILE_BASE_URL);
        let job_url = format!("{0}/{id}/class_job", PROFILE_BASE_URL);

        let (profile_html, job_html) = info_span!("download").in_scope(|| {
            let profile_html = ureq::get(profile_url.as_str())
                .call()
                .map_err(|e| e.to_string())?
                .into_string()
                .map_err(|e| e.to_string())?;
            let job_html = ureq::get(job_url.as_str())
                .call()
                .map_err(|e| e.to_string())?
                .into_string()
                .map_err(|e| e.to_string())?;
            event!(
                Level::INFO,
                "downloaded html successfully for {name} ({id})"
            );
            Ok::<(String, String), String>((profile_html, job_html))
        })?;
        info_span!("write").in_scope(|| {
            fs::write(profile, profile_html.clone()).map_err(|e| e.to_string())?;
            fs::write(job, job_html).map_err(|e| e.to_string())?;
            event!(
                Level::INFO,
                "wrote updated html successfully for {name} ({id})"
            );
            Ok::<(), String>(())
        })
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Download/update files for running tests.
    SetupTests { output: PathBuf },
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    tracing_subscriber::fmt::init();

    match cli.command {
        Command::SetupTests { output } => setup_tests(output),
    }
}

#[instrument(skip_all)]
fn setup_tests(output_directory: PathBuf) -> Result<(), String> {
    event!(Level::INFO, "loading test profile list");
    let profiles = knuffel::parse::<Vec<TestProfile>>("test-profiles.kdl", PROFILES).unwrap();
    event!(Level::INFO, "downloading profiles");
    for profile in profiles {
        profile.fetch(output_directory.as_path())?;
    }
    event!(Level::INFO, "completed successfully!");
    Ok(())
}
