# jogger-core

Core library for Jogger - provides Jira time logging functionality, time parsing, and preferences management.

## Features

- **Jira Integration**: Submit time logs to Jira via REST API
- **Time Parsing**: Parse various time formats (1h30m, 1.5h, 90m, etc.)
- **Preferences Management**: Load and save user configuration
- **Meeting Types**: Predefined meeting categories and projects

## Usage

```rust
use jogger_core::{Preferences, TimeLog, submit_timelog, string_to_seconds};
use std::{cell::RefCell, rc::Rc};

// Load preferences
let prefs = Rc::new(RefCell::new(Preferences::load().unwrap_or_default()));

// Parse time
let seconds = string_to_seconds("1h30m").unwrap(); // 5400 seconds

// Submit time log
let log = TimeLog {
    time_spent_seconds: seconds,
    comment: "Working on feature".to_string(),
    ticket_number: "PROJ-123".to_string(),
    prefs: prefs.clone(),
};

submit_timelog(&log).unwrap();
```

## Time Format Examples

- `1h` - 1 hour
- `30m` - 30 minutes
- `1h30m` - 1 hour 30 minutes
- `1h30` - 1 hour 30 minutes (implicit m)
- `1.5h` - 1.5 hours
- `1.5` - 1.5 hours (implicit h)
