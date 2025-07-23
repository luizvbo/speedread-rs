## speedread-rs

`speedread-rs` is a minimalist command-line application designed
to help you read faster using the Rapid Serial Visual Presentation (RSVP)
method. It displays one word at a time at a customizable speed,
helping to reduce subvocalization and improve reading efficiency.
The application is built with Rust, ensuring high performance and
reliability.

## Features

  - **Customizable WPM (Words Per Minute)**: Adjust your reading speed on the fly.
  - **Dynamic Word Timing**: Optionally adjusts display time based on word length for better readability.
  - **Pause Functionality**: Pause and resume reading at any time.
  - **Context View**: When paused, view the surrounding text to regain context.
  - **Terminal-Based UI**: A clean and distraction-free reading experience directly in your terminal.
  - **Reads from File or Stdin**: Easily read content from a specified file or pipe text directly into the application.


## Installation

To install `speedread-rs`, you'll need Rust and Cargo installed on your system. If you don't have them, you can install them via `rustup`.

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/luizvbo/speedread-rs.git
    cd speedread-rs
    ```
2.  **Build the project:**
    ```bash
    cargo build --release
    ```
3.  **Run the application:**
    The executable will be located in `./target/release/speedread-rs`. You can add it to your `PATH` for easier access, or run it directly.

## Usage

You can use `speedread-rs` by providing a file path or by piping text into it.

### Reading from a file

```bash
speedread-rs -f your_document.txt
```

### Reading from stdin

```bash
cat your_document.txt | speedread-rs
```

Or simply type your text and press `Ctrl+D` (or `Ctrl+Z` on Windows) to signal EOF:

```bash
speedread-rs
This is some text I want to speed read.
^D
```

### Command Line Arguments

  * `-f, --file <FILE>`: The file to read from. If not provided, `speedread-rs` reads from stdin.
  * `--wpm <WPM>`: Initial words per minute (default: 250.0).
  * `-d, --dynamic-time`: Calculate word display time based on its length.

**Example:**

Start reading `article.txt` at 300 WPM with dynamic timing:

```bash
speedread-rs -f article.txt --wpm 300 -d
```

## Controls

While `speedread-rs` is running, you can use the following keyboard controls:

  * **Spacebar**: Toggle pause/resume.
  * **+ / =**: Increase WPM by 10 (max 1000 WPM).
  * **-**: Decrease WPM by 10 (min 10 WPM).
  * **Esc / q**: Quit the application.

## Contributing

Contributions are welcome\! If you have any suggestions, bug reports, or want to contribute code, please feel free to open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License - see the `LICENSE` file for details.


