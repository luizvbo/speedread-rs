use crossterm::event::{self, Event, KeyCode};
use std::sync::{mpsc::Sender, Arc, Mutex};
use std::time::Duration;

pub fn handle_keyboard_input(
    wpm: Arc<Mutex<f64>>,
    paused: Arc<Mutex<bool>>,
    quit_sender: Sender<()>,
) {
    loop {
        // Poll for a key event for a short duration
        if event::poll(Duration::from_millis(100)).unwrap_or(false) {
            // If a key event is available, read it
            if let Ok(Event::Key(key)) = event::read() {
                let mut paused_lock = paused.lock().unwrap();
                let mut wpm_lock = wpm.lock().unwrap();

                match key.code {
                    // Toggle pause
                    KeyCode::Char(' ') => *paused_lock = !*paused_lock,
                    // Increase WPM
                    KeyCode::Char('+') | KeyCode::Char('=') => *wpm_lock = (*wpm_lock + 10.0).min(1000.0),
                    // Decrease WPM
                    KeyCode::Char('-') => *wpm_lock = (*wpm_lock - 10.0).max(10.0),
                    // Quit
                    KeyCode::Esc | KeyCode::Char('q') => {
                        if quit_sender.send(()).is_ok() {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
