mod app;
mod config;
mod ui;
mod input;

use config::Args;
use clap::Parser;
use std::io::{self, Read};
use std::sync::{Arc, Mutex};
use unicode_segmentation::UnicodeSegmentation;

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut text = String::new();

    // Read text from file or stdin
    if let Some(file_path) = &args.file {
        text = std::fs::read_to_string(file_path)?;
    } else {
        io::stdin().read_to_string(&mut text)?;
    }

    let words: Vec<&str> = text.unicode_words().collect();
    if words.is_empty() {
        println!("No words to display.");
        return Ok(());
    }

    // Initialize shared state
    let wpm = Arc::new(Mutex::new(args.wpm));
    let paused = Arc::new(Mutex::new(false));

    // Run the application
    app::run_rsvp(&words, wpm, paused, args.dynamic_time)?;

    Ok(())
}
