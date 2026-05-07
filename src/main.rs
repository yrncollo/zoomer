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
