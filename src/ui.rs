use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use std::io::{self, Stdout};

/// Calculates the Optimal Recognition Point (ORP) index for a word.
fn find_orp(word: &str) -> usize {
    let len = word.chars().count();
    match len {
        0..=1 => 0,
        2..=4 => 1,
        5..=8 => 2,
        9..=12 => 3,
        _ => 4,
    }
}

/// Displays a single word centered on the ORP.
pub fn display_word(stdout: &mut Stdout, word: &str, wpm: f64) -> io::Result<()> {
    let (width, height) = terminal::size()?;
    let orp_visual_pos = width / 2;
    let vertical_pos = height / 2;

    let orp_index = find_orp(word);
    let (pre, rest) = word.split_at(orp_index);
    let pivot = rest.chars().next().unwrap_or(' ');
    let post = &rest[pivot.len_utf8()..];
    let word_start_pos = orp_visual_pos.saturating_sub(orp_index as u16);

    stdout.execute(Clear(ClearType::All))?;

    // Draw top guide '▼'
    stdout
        .execute(cursor::MoveTo(
            orp_visual_pos,
            vertical_pos.saturating_sub(1),
        ))?
        .execute(SetForegroundColor(Color::Red))?
        .execute(Print("▼"))?
        .execute(ResetColor)?;

    // Draw the word itself
    stdout
        .execute(cursor::MoveTo(word_start_pos, vertical_pos))?
        .execute(Print(pre))?
        .execute(SetForegroundColor(Color::Red))?
        .execute(Print(pivot))?
        .execute(ResetColor)?
        .execute(Print(post))?;

    // Draw WPM status
    let wpm_text = format!("{:.0} WPM", wpm);
    stdout
        .execute(cursor::MoveTo(
            width.saturating_sub(wpm_text.len() as u16),
            0,
        ))?
        .execute(Print(wpm_text))?;

    Ok(())
}

/// Displays the context view when paused.
pub fn display_context(
    stdout: &mut Stdout,
    words: &[&str],
    current_index: usize,
    wpm: f64,
) -> io::Result<()> {
    let (width, height) = terminal::size()?;
    stdout.execute(Clear(ClearType::All))?;

    // Draw WPM status
    let wpm_text = format!("{:.0} WPM", wpm);
    stdout
        .execute(cursor::MoveTo(
            width.saturating_sub(wpm_text.len() as u16),
            0,
        ))?
        .execute(Print(wpm_text))?;

    // Draw "PAUSED" status
    let paused_text = "PAUSED";
    stdout
        .execute(cursor::MoveTo(
            (width.saturating_sub(paused_text.len() as u16)) / 2,
            height / 2 - 2,
        ))?
        .execute(SetForegroundColor(Color::Yellow))?
        .execute(Print(paused_text))?
        .execute(ResetColor)?;

    // Draw context lines
    let start = current_index.saturating_sub(20);
    let end = (current_index + 20).min(words.len());
    let context_words = &words[start..end];
    let mut current_line = String::new();
    let mut y = height / 2;

    for (i, &word) in context_words.iter().enumerate() {
        if current_line.len() + word.len() + 1 > width as usize {
            stdout
                .execute(cursor::MoveTo(0, y))?
                .execute(Print(&current_line))?;
            current_line.clear();
            y += 1;
            if y >= height.saturating_sub(1) {
                break;
            }
        }

        if start + i == current_index {
            current_line.push_str("\x1B[7m"); // ANSI reverse video on
            current_line.push_str(word);
            current_line.push_str("\x1B[0m "); // ANSI reverse video off + space
        } else {
            current_line.push_str(word);
            current_line.push(' ');
        }
    }
    
    stdout
        .execute(cursor::MoveTo(0, y))?
        .execute(Print(current_line))?;

    Ok(())
}
