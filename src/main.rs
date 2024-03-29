use std::{env, mem};
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

    // Wrap logger inside sentry
    let logger = sentry_log::SentryLogger::with_dest(builder.build());
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log_level);
    trace!("debug is on");

    if let Ok(dsn) = env::var("SENTRY_DSN") {
        let guard = sentry::init(dsn);
        if guard.is_enabled() {
            info!("Sentry enabled");
            mem::forget(guard)  // Used to keep the guard active
        }
    }

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