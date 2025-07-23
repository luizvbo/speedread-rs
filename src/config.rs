use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "A terminal-based speed reader.", long_about = None)]
pub struct Args {
    /// The file to read from. If not provided, reads from stdin.
    pub file: Option<String>,

    /// Initial words per minute
    #[arg(short, long, default_value_t = 250.0)]
    pub wpm: f64,

    /// Calculate word display time based on its length
    #[arg(short = 'd', long)]
    pub dynamic_time: bool,
}
