# editor-dojo

A gamified terminal-based training tool for mastering text editors like Helix, Vim, Neovim, and Emacs.

## Overview

editor-dojo helps you practice and improve your text editing skills through interactive challenges. Complete editing tasks as quickly as possible while the tool tracks your performance and progress.

## Features

- **Interactive Challenges**: Transform text from a starting state to a target state
- **Automatic Validation**: The editor closes automatically when you complete the challenge
- **Performance Tracking**: See how long it took you to complete each challenge
- **Keystroke Recording**: Records your editing session with asciinema
- **Key Sequence Display**: See exactly what keys you pressed to complete the challenge
- **Session Replay**: Review your editing sessions with asciinema playback
- **Clean Architecture**: Built with SOLID principles for easy extension

## Prerequisites

### Required
- Rust (2021 edition or later)
- [Helix editor](https://helix-editor.com/) (`hx` command must be in PATH)

### Recommended
- [asciinema](https://asciinema.org/) for keystroke recording and feedback

**Note**: The tool works without asciinema, but you won't see keystroke counts, key sequences, or session recordings.

## Installation

### Install Dependencies

**Install Helix** (required):
```bash
# See https://helix-editor.com/ for installation instructions
```

**Install asciinema** (recommended):
```bash
# macOS
brew install asciinema

# Debian/Ubuntu
sudo apt install asciinema

# Fedora
sudo dnf install asciinema

# Arch
sudo pacman -S asciinema

# For other systems, see: https://asciinema.org/docs/installation
```

### Build editor-dojo

```bash
cargo build --release
```

## Usage

```bash
cargo run
```

1. Select a challenge from the list
2. Read the challenge description
3. Complete the editing task in Helix
4. The editor will close automatically when you succeed
5. View your results including time, keystrokes, and key sequence

## How It Works

### Recording Mechanism

When you start a challenge, editor-dojo wraps your editor session with asciinema:
```bash
asciinema rec --overwrite <output.cast> -c "hx <file>"
```

All your keystrokes and terminal output are recorded in the `.cast` file format.

### Where Recordings Are Stored

Recordings are saved to:
```
~/.local/share/editor-dojo/recordings/
```

Each recording is named:
```
challenge-{id}-{timestamp}.cast
```

### Replaying Your Sessions

To watch a replay of your session:
```bash
asciinema play ~/.local/share/editor-dojo/recordings/challenge-<id>-<timestamp>.cast
```

**Privacy Note**: Recordings contain all keystrokes entered during the session.

## Understanding Results

After completing a challenge, you'll see:

```
┌───────────────────────────────────────────────────────┐
│              ✓ CHALLENGE COMPLETE!                    │
├───────────────────────────────────────────────────────┤
│                                                       │
│  Time:        0:08s                                   │
│  Keystrokes:  12                                      │
│                                                       │
│  Key sequence:                                        │
│    w w d w Esc : q Enter                              │
│                                                       │
│  Recording: ~/.local/share/editor-dojo/...            │
│  Replay: asciinema play <path>                        │
│                                                       │
│  [ Press any key to exit ]                            │
└───────────────────────────────────────────────────────┘
```

### Key Representations

- **Regular keys**: Shown as-is (`a`, `b`, `1`, `2`, etc.)
- **Special keys**: Human-readable names
  - `Enter` - Return/Enter key
  - `Esc` - Escape key
  - `Space` - Spacebar
  - `Tab` - Tab key
  - `Backspace` - Backspace key
  - Arrow keys: `Up`, `Down`, `Left`, `Right`
- **Ctrl combinations**: `Ctrl-c`, `Ctrl-d`, etc.
- **Alt combinations**: `Alt-f`, `Alt-b`, etc.

The sequence shows every key you pressed in order, space-separated for readability.

## Troubleshooting

### asciinema not found

**Symptom**: You see a message saying asciinema is not installed.

**Solution**:
1. Install asciinema following the instructions in the Installation section above
2. Ensure it's in your PATH by running: `asciinema --version`
3. Restart editor-dojo

**Workaround**: You can continue without asciinema, but recording features will be disabled.

### Permission issues with recordings directory

**Symptom**: Error creating or writing to recordings directory.

**Solution**:
```bash
mkdir -p ~/.local/share/editor-dojo/recordings
chmod 755 ~/.local/share/editor-dojo/recordings
```

### Corrupted or unparseable recording files

**Symptom**: Warning message about failed recording parse, but challenge still completes.

**Impact**: The key sequence won't be displayed, but the recording file is still saved.

**Solution**:
- You can still replay the recording with `asciinema play <path>`
- If the .cast file is corrupted, try running the challenge again

### Missing key sequences in results

**Symptom**: Results screen shows no keystroke count or key sequence.

**Possible causes**:
1. asciinema is not installed (check startup message)
2. Recording failed during the session (check for warnings)
3. Parse error (a warning message would have been shown)

**Solution**: Ensure asciinema is installed and try the challenge again.

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
│   ├── solution.rs             # Solution value object
│   ├── recording.rs            # Recording value object
│   └── key_sequence.rs         # Key sequence value object
├── application/
│   ├── challenge_runner.rs     # Orchestrates the challenge flow
│   └── validator.rs            # Solution validation logic
├── infrastructure/
│   ├── editor.rs               # Helix editor spawner
│   ├── watcher.rs              # File change watcher
│   ├── filesystem.rs           # File system operations
│   ├── recorder.rs             # Asciinema recorder implementation
│   ├── cast_parser.rs          # .cast file parser for keystroke extraction
│   └── challenge_loader.rs     # TOML challenge loader
└── ui/
    ├── challenge_list_screen.rs # Challenge selection TUI
    ├── challenge_screen.rs      # Challenge brief TUI
    └── results_screen.rs        # Results display with key sequences
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

- Support for multiple editors (Vim, Neovim, Emacs)
- Semantic command interpretation (e.g., "dw" → "delete word")
- Command frequency analysis and optimization hints
- Progress tracking and personal bests
- Community leaderboards
- Solution sharing across editors
- Macro detection and display
- Side-by-side comparison of multiple attempts
- Optimal solution suggestions

## License

See LICENSE file for details.