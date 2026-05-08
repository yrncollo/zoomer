use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use clap::Parser;

mod app;
mod capture;
mod config;
mod input;
mod math;
mod renderer;
mod window;

use crate::app::{run, AppConfig};
use crate::config::Config;

/// zoomer — screen magnifier for X11, inspired by boomer (Nim)
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value_t = 0.0)]
    delay: f64,

    #[arg(short, long)]
    windowed: bool,

    #[arg(short, long)]
    config: Option<PathBuf>,

    #[arg(long, value_name = "PATH")]
    new_config: Option<Option<PathBuf>>,
}

fn default_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("zoomer")
        .join("config")
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if let Some(maybe_path) = args.new_config {
        let path = maybe_path.unwrap_or_else(default_config_path);
        if path.exists() {
            eprint!("File {} already exists. Replace it? [yn] ", path.display());
            let mut answer = String::new();
            std::io::stdin().read_line(&mut answer)?;
            if !answer.trim().eq_ignore_ascii_case("y") {
                eprintln!("Aborted.");
                return Ok(());
            }
        }
        Config::write_default(&path)?;
        println!("Generated config at {}", path.display());
        return Ok(());
    }

    if args.delay > 0.0 {
        thread::sleep(Duration::from_millis((args.delay * 1000.0) as u64));
    }

    let config_path = args.config.unwrap_or_else(default_config_path);
    let config = if config_path.exists() {
        eprintln!("Loading config from {}", config_path.display());
        Config::load(&config_path)?
    } else {
        eprintln!(
            "{} not found — using defaults.",
            config_path.display()
        );
        Config::default()
    };

    eprintln!("Config: {:?}", config);

    run(AppConfig {
        config,
        windowed: args.windowed,
        config_path,
    })
}
