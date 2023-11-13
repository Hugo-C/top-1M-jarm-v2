use clap::{Parser, Subcommand};
use env_logger::{Builder, Target};
use log::{info, LevelFilter, trace, warn};
use top_1m_jarm_v2::{run_scheduler, run_uploader, run_worker};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    dry_run: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scheduler,
    Worker,
    Uploader,
}

fn main() {
    let cli = Cli::parse();
    let mut builder = Builder::new();
    let log_level = if cli.verbose {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    };
    builder.filter_level(log_level);
    builder.format_timestamp(None);
    builder.format_target(false);
    builder.target(Target::Stdout);
    builder.init();
    trace!("debug is on");

    match &cli.command {
        Commands::Scheduler => {
            info!("Starting scheduler");
            if cli.dry_run {
                warn!("Dry run, local tranco list sample used");
            }
            run_scheduler(cli.dry_run);
            info!("Scheduler done!");
        }
        Commands::Worker => {
            info!("Starting worker");
            if cli.dry_run {
                warn!("Dry run, no real jarm hash will be returned");
            }
            run_worker(cli.dry_run);
            info!("Worker done!");
        }
        Commands::Uploader => {
            info!("Starting uploader");
            if cli.dry_run {
                warn!("Dry run, nothing will be uploaded to S3");
            }
            run_uploader(cli.dry_run);
            info!("uploader done!");
        }
    }
}