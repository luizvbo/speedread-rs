# speedread-rs

**speedread-rs** is a terminal-based speed reading tool written in Rust. It
displays one word at a time using the Rapid Serial Visual Presentation (RSVP)
technique, helping users read text faster and with better focus.

## 🚀 Features

-  Reads from a file or standard input
-  Adjustable Words Per Minute (WPM)
-  Dynamic timing based on word length
-  Pause and resume with context view
-  Optimal Recognition Point (ORP) highlighting
-  Clean terminal UI using `crossterm`

## 📦 Installation

Make sure you have https://www.rust-lang.org/tools/install installed.

```bash
git clone https://github.com/luizvbo/speedread-rs.git
cd speedread-rs
cargo build --release
```

## 🧪 Usage

```bash
speedread-rs [OPTIONS] [FILE]
```

### Options

- `-w`, `--wpm <WPM>`: Set initial words per minute (default: 250)
- `-d`, `--dynamic-time`: Enable dynamic timing based on word length
- `-h`, `--help`: Show help message

### Examples

```bash
# Read from a file at 300 WPM
speedread-rs -w 300 sample.txt

# Read from stdin with dynamic timing
cat sample.txt | speedread-rs -d
```

## ⌨️ Controls

- `Space`: Pause/Resume
- `+` or `=`: Increase WPM
- `-`: Decrease WPM
- `q` or `Esc`: Quit

## 🛠️ Dependencies

- https://crates.io/crates/clap – Command-line argument parsing
- https://crates.io/crates/crossterm – Terminal manipulation
- https://crates.io/crates/unicode-segmentation – Unicode word boundaries

## 📁 Project Structure

- `main.rs`: Entry point, handles input and app setup
- `config.rs`: CLI argument parsing
- `ui.rs`: Terminal rendering logic
- `input.rs`: Keyboard event handling
- `app.rs`: Main application loop

## 📜 License

This project is licensed under the MIT License. See the LICENSE file for details.
