# memer.rs

A Rust CLI tool for generating memes using OpenAI's DALL-E 3 API.

[![Docker Image](https://img.shields.io/badge/Docker-Hub-blue?logo=docker)](https://hub.docker.com/r/programmerrdai/memer)
[![Crates.io](https://img.shields.io/crates/v/memer-rs.svg)](https://crates.io/crates/memer-rs)

## Description

memer.rs is a command-line interface that allows you to generate meme images by simply providing a text query. The tool uses OpenAI's DALL-E 3 model to create internet-style memes with bold, impact font text and high-contrast visuals.

## Features

- Fast meme generation using DALL-E 3
- Automatic UUID-based file organization
- Optimized for classic meme format (top/bottom text, bold styling)
- Async/await for efficient API calls
- Configurable output directories

## Installation

### Prerequisites

- Rust (latest stable version)
- OpenAI API key

### From Source

```bash
git clone https://github.com/Programmer-RD-AI/memer.rs.git
cd memer.rs
cargo build --release
```

### From Crates.io

```bash
cargo install memer-rs
```

## Setup

1. Get an OpenAI API key from [OpenAI Platform](https://platform.openai.com/)
2. Create a `.env` file in the project root:

```env
OPENAI_API_KEY=your_api_key_here
```

## Usage

### Basic Usage

```bash
memer --query "when you finally understand Rust ownership"
```

### Custom Output Directory

```bash
memer --query "debugging at 3am" --folder my_memes
```

### Help

```bash
memer --help
```

## Options

- `-q, --query <QUERY>`: The meme description/topic (required)
- `-f, --folder <FOLDER>`: Output directory (default: random UUID)

## Examples

```bash
# Generate a programming meme
memer -q "when your code works on the first try"

# Generate a relatable meme with custom folder
memer -q "me explaining why I need another monitor" -f work_memes

# Generate a Rust-specific meme
memer -q "borrowing in Rust vs other languages"
```

## Output

Generated images are saved as 1024x1024 PNG files in the specified directory. The tool automatically creates the directory if it doesn't exist.

## Technical Details

- **Language**: Rust 2024 edition
- **Image Model**: DALL-E 3
- **Image Format**: Base64 JSON → PNG
- **Image Size**: 1024x1024 pixels
- **Async Runtime**: Tokio

## Dependencies

- `async-openai`: OpenAI API client
- `clap`: Command-line argument parsing
- `tokio`: Async runtime
- `dotenv`: Environment variable loading
- `minijinja`: Template rendering
- `uuid`: Unique identifier generation

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

Apache License 2.0

## Acknowledgments

- OpenAI for the DALL-E 3 API
- The Rust community for excellent crates

---

**Note**: This tool requires an active OpenAI API subscription. Image generation costs apply according to OpenAI's pricing.
