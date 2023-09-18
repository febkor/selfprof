use std::string::String;

use clap::Parser;

/// Simple time tracker.
#[derive(Parser, Debug)]
#[command(author = "febkor", version = "1", about, long_about = None)]
pub struct Config {
    /// Directory where to store snaps.
    #[arg(short, long, default_value = "~/selfprof")]
    pub out_dir: String,

    /// Seconds between each snap.
    #[arg(long, default_value_t = 10)]
    pub interval_snap: u32,

    /// Seconds between each save.
    #[arg(long, default_value_t = 60)]
    pub interval_save: u32,

    /// Seconds of idling after which profiling pauses.
    #[arg(long, default_value_t = 60*60*2)]
    pub idle_cutoff: u16,

    /// Print debug info.
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

pub fn parse() -> Config {
    let args = Config::parse();
    if args.verbose {
        dbg!(&args);
    }
    args
}
