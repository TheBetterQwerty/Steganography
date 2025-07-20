# Stegnograph

**Stegnograph** is a command-line utility for embedding and extracting hidden data within media files using steganographic techniques. It is currently focused on **image-based steganography**, with planned support for audio and video formats in future updates.

---

## Features

* Embed and extract data within image files (`.png`, `.bmp`, etc.)
* Optional custom header string for metadata or identification
* Simple, Unix-style command-line interface
* Built in Rust for performance and safety

---

## Installation

Ensure Rust is installed

Then clone and build the project:

```sh
git clone https://github.com/your-username/stegnograph.git
cd stegnograph
cargo build --release
```

The compiled binary will be located at `target/release/stegnograph`.

---

## Usage

```sh
stegnograph [--embedd | --uproot] --image <FILE> -o <OUTPUT> [--header <STRING>]
```

> **Note**: Only the `--image` option is currently implemented. Support for audio and video files is planned.

### Options

| Option                | Description                        |
| --------------------- | ---------------------------------- |
| `--image <FILE>`      | Input image file                   |
| `--embedd`            | Embed data into the image          |
| `--uproot`            | Extract hidden data from the image |
| `-o, --output <FILE>` | Output file path                   |
| `--header <DATA>`     | Optional header string to embed    |
| `--help`              | Display help message               |
| `--version`           | Display version information        |

---

## Examples

Embed data into an image:

```sh
stegnograph --embedd --image input.png -o output.png --header "confidential"
```

Extract data from an image:

```sh
stegnograph --uproot --image output.png -o message.txt
```

---

## Planned Updates

The following features are under consideration for future releases:

* Support for audio file embedding and extraction (e.g., WAV)
* Support for video file embedding and extraction (e.g., MP4)
* More advanced encoding modes and error handling

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---
