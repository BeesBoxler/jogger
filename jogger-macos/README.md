# Jogger macOS

Native macOS application for Jogger using Cacao (AppKit bindings).

## Status

✅ **FULLY FUNCTIONAL** - Complete Jira integration with native macOS UI!

## Features

- 🎨 Native macOS interface with AppKit
- ⚡ Full Jira time logging integration via `jogger-core`
- 🕐 Smart time parsing (1h30m, 1.5h, 90m, etc.)
- 🔄 Async submission (doesn't block UI)
- ✨ Native menu bar and keyboard shortcuts

## Running

```bash
cargo run --release -p jogger-macos
```

## How It Works

The app uses some clever Rust magic to work around Cacao's `Send + Sync` requirements:

1. **Arc<Mutex<T>>** for thread-safe state management
2. **Raw pointers** to pass non-Send UI elements into closures
3. **Thread spawning** for API calls (keeps Rc/RefCell in the worker thread)

This proves the `jogger-core` library architecture works perfectly - the same business logic powers both the TUI and this native macOS app!

## Configuration

Uses the same config as the TUI version: `~/.config/jogger.conf`

Configure your Jira URL, email, and API token there first.

