# Jogger 🏃🏼‍♀️

A simple application for logging time to Jira tickets. Because time logging has far too much friction and I get in far too much trouble for not doing it.

Simple Jira time logging with two interfaces:
- `jogger-macos`: macOS menu bar app
- `jogger` (`jogger-tui`): terminal UI

Both use the same config file: `~/.config/jogger.conf`.

## Who This Is For

If your team logs time in Jira and wants low-friction prompts + fast ticket logging.

## Jira Access Requirements

You need:
- Jira base URL (for example `https://jira.company.com`)
- Jira account email
- Jira Personal Access Token / API token with full access
  - Scoped/restricted tokens are not currently supported by this app's Jira calls.

For your colleagues: a full-access key will work if your Jira setup requires broad permissions.

## Install

### Preferred (macOS): Homebrew Cask

```bash
brew install --cask beesboxler/jogger/jogger-macos
xattr -dr com.apple.quarantine /Applications/Jogger.app
```

### Option A: Download from GitHub Releases

- macOS: download `jogger-macos-<apple-target>.zip`, unzip, move `Jogger.app` to `~/Applications` or `/Applications`
- Linux: download `jogger-<linux-target>.tar.gz`, extract, put binary somewhere on `PATH`

Releases: <https://github.com/BeesBoxler/jogger/releases>

### Option B: Build/install with Cargo

```bash
# TUI
cargo install jogger

# macOS menu bar app
cargo install jogger-macos
```

## First Run Setup

### macOS App

1. Launch `Jogger.app`.
2. Open menu bar icon -> `Preferences`.
3. Fill in:
   - Name (optional)
   - Email
   - API Key / Token
   - Jira URL
4. Save.

### TUI

1. Run `jogger`.
2. Open `Setup`.
3. Enter the same values and save.

## Configuration File

Path: `~/.config/jogger.conf`

You do not need to create this file manually. Jogger creates it on first run when you save setup/preferences. Most people only need to edit `custom_meetings` later.

Example:

```json
{
  "name": "Jane Doe",
  "email": "jane@company.com",
  "api_key": "jira_token_here",
  "jira_url": "https://jira.company.com",
  "custom_meetings": [
    {
      "name": "ProjectA",
      "meetings": [
        ["Billable", "PROJA-1234"],
        ["NonBillable", "ADMIN-42"],
        ["Deployment", "OPS-900"]
      ]
    }
  ],
  "reminder_settings": {
    "enabled": true,
    "interval_minutes": 30
  },
  "timer_state": {
    "last_log_time": null,
    "accumulated_seconds": 0,
    "last_ticket": null,
    "last_log_date": null
  }
}
```

Notes:
- `interval_minutes` supports `15`, `30`, or `60`.
- `custom_meetings` powers distraction/quick-pick ticket menus.
- You normally should not hand-edit `timer_state`.

## macOS Auto Start at Login

```bash
./install-launchagent.sh
```

Uninstall:

```bash
launchctl unload ~/Library/LaunchAgents/com.jogger.macos.plist
rm ~/Library/LaunchAgents/com.jogger.macos.plist
```

## Time Input Formats

Accepted formats include:
- `1h`
- `30m`
- `1h30m`
- `1h30`
- `1.5h`
- `1.5`

## Troubleshooting

- macOS says app is from an unknown developer:
  - Right click app -> `Open`, or use System Settings -> Privacy & Security -> `Open Anyway`.
- `401`/`403` from Jira:
  - Check email/token pair
  - Confirm token permissions for worklog creation
  - Verify Jira URL is correct (no extra path)
- No distraction tickets shown:
  - Add entries in `custom_meetings` (or use defaults)

## Development

```bash
# Run tests
cargo test

# Run TUI
cargo run -p jogger

# Run macOS menu bar app
cargo run -p jogger-macos
```
