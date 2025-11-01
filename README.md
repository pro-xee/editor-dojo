# editor-dojo

A gamified terminal-based training tool for mastering text editors like Helix, Vim, Neovim, and Emacs.

## Overview

editor-dojo helps you practice and improve your text editing skills through interactive challenges. Complete editing tasks as quickly as possible while the tool tracks your performance and progress.

## Features (MVP)

- **Interactive Challenges**: Transform text from a starting state to a target state
- **Automatic Validation**: The editor closes automatically when you complete the challenge
- **Performance Tracking**: See how long it took you to complete each challenge
- **Clean Architecture**: Built with SOLID principles for easy extension

## Prerequisites

- Rust (2021 edition or later)
- [Helix editor](https://helix-editor.com/) (`hx` command must be in PATH)

## Installation

```bash
cargo build --release
```

## Usage

```bash
cargo run
```

The current MVP includes a single hardcoded challenge:
- **Challenge**: Delete the word REMOVE
- **Starting text**: "Hello REMOVE world"
- **Target text**: "Hello world"

## Architecture

This project follows Clean Architecture (Hexagonal/Onion) principles:

```
├── domain/          # Core business entities (Challenge, Solution)
├── application/     # Business logic (ChallengeRunner, Validator)
├── infrastructure/  # External adapters (Editor, FileWatcher, FileSystem)
└── ui/              # User interface (TUI screens)
```

### Dependency Rules

- **Domain layer**: No external dependencies
- **Application layer**: Depends only on domain
- **Infrastructure layer**: Implements application interfaces
- **UI layer**: Presents information to users

## Project Structure

```
src/
├── main.rs                      # Entry point with dependency injection
├── domain/
│   ├── challenge.rs            # Challenge entity
│   └── solution.rs             # Solution value object
├── application/
│   ├── challenge_runner.rs     # Orchestrates the challenge flow
│   └── validator.rs            # Solution validation logic
├── infrastructure/
│   ├── editor.rs               # Helix editor spawner
│   ├── watcher.rs              # File change watcher
│   └── filesystem.rs           # File system operations
└── ui/
    ├── challenge_screen.rs     # Challenge brief TUI
    └── results_screen.rs       # Results display TUI
```

## Development

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
cargo fmt
cargo clippy
```

## Future Features

- Multiple challenges loaded from TOML files
- Support for multiple editors (Vim, Neovim, Emacs)
- asciinema recording and playback
- Keystroke and command analysis
- Progress tracking and personal bests
- Community leaderboards
- Solution sharing across editors

## License

See LICENSE file for details.