# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run

```bash
# Build (macOS/Linux)
./build.sh

# Build for Windows (cross-compile)
./build_windows.sh

# Run
./ch

# Run tests
cargo test
```

## Architecture

The project follows a clean modular architecture typical of Ratatui TUI applications:

```
src/
├── main.rs          # Entry point, terminal setup, main loop
├── app.rs           # Application state (App struct)
├── events.rs        # Input event handling
├── data/
│   └── mod.rs       # Built-in entries data (shortcuts, commands)
└── ui/
    ├── mod.rs       # UI module entry point
    ├── layout.rs    # TUI layout creation
    ├── tabs.rs      # Tab rendering
    ├── popup.rs     # Search popup overlay
    └── status_bar.rs # Bottom status bar
```

### Core Components

- **App** (`app.rs`): Manages application state including current section, selection index, scroll offsets, input mode (Normal/Searching), and search query.
- **Event Handling** (`events.rs`): Processes keyboard input via crossterm, routing to normal or search mode handlers.
- **Data** (`data/mod.rs`): Contains `BuiltinEntry` definitions organized by `Category` (Shortcuts, SlashCommands, CliCommands).
- **UI** (`ui/`): Render functions for the three-column layout, tabs, search popup, and status bar.

### Main Loop

The application runs a standard TUI loop:
1. Initialize terminal with raw mode and alternate screen
2. Create `App` instance
3. Loop: render UI → handle events → check quit flag
4. Restore terminal on exit

## Key Patterns

- **Section navigation**: 3 sections (0-2), cyclic navigation via `next_section()`/`prev_section()`
- **Search filtering**: Toggle with `toggle_search()`, updates `input_mode` state
- **Copy feedback**: `copy_feedback` field stores feedback message for status bar display
