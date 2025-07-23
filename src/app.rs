use crate::{input, ui};
use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::{self, stdout};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn calculate_sleep_duration(word: &str, wpm: f64, dynamic: bool) -> Duration {
    let base_millis = 60_000.0 / wpm;
    if dynamic {
        let len = word.chars().count() as f64;
        // A simple heuristic for dynamic timing
        let scale_factor = 0.8 + (len * 0.05);
        let min_millis = 150.0; // A lower bound so very short words are still readable
        Duration::from_millis(base_millis.mul_add(scale_factor, min_millis) as u64)
    } else {
        Duration::from_millis(base_millis as u64)
    }
}

pub fn run_rsvp(
    words: &[&str],
    wpm: Arc<Mutex<f64>>,
    paused: Arc<Mutex<bool>>,
    dynamic_time: bool,
) -> io::Result<()> {
    let wpm_clone = Arc::clone(&wpm);
    let paused_clone = Arc::clone(&paused);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || input::handle_keyboard_input(wpm_clone, paused_clone, tx));

    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout
        .execute(terminal::EnterAlternateScreen)?
        .execute(cursor::Hide)?;

    let mut current_word_index = 0;
    'reading_loop: while current_word_index < words.len() {
        // Check if the input thread has sent a quit signal
        if rx.try_recv().is_ok() {
            break 'reading_loop;
        }

        // Now we can use the original `wpm` and `paused` Arcs without error
        let is_paused = *paused.lock().unwrap();
        let current_wpm = *wpm.lock().unwrap();

        if is_paused {
            ui::display_context(&mut stdout, words, current_word_index, current_wpm)?;
            thread::sleep(Duration::from_millis(50)); // Wait while paused
            continue;
        }

        let word = words[current_word_index];
        let sleep_duration = calculate_sleep_duration(word, current_wpm, dynamic_time);

        ui::display_word(&mut stdout, word, current_wpm)?;

        // Wait for the word's display duration, while also checking for pause/quit events
        let start_time = Instant::now();
        while start_time.elapsed() < sleep_duration {
            if rx.try_recv().is_ok() {
                break 'reading_loop;
            }
            if *paused.lock().unwrap() {
                continue 'reading_loop; // Exit sleep and re-enter outer loop to show context
            }
            thread::sleep(Duration::from_millis(10));
        }

        if !*paused.lock().unwrap() {
            current_word_index += 1;
        }
    }

    // Cleanup terminal state
    stdout
        .execute(terminal::LeaveAlternateScreen)?
        .execute(cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
