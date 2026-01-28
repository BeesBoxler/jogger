use serde::{Deserialize, Serialize};

use std::io::Error;
use std::{cell::RefCell, rc::Rc};
use time::OffsetDateTime;

use crate::meeting_types::{seed_meeting_tickets, Project};

const PREF_FILENAME: &str = "jogger.conf";

pub type PrefRef = Rc<RefCell<Preferences>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderSettings {
    pub enabled: bool,
    pub interval_minutes: u32, // 15, 30, or 60
}

impl Default for ReminderSettings {
    fn default() -> Self {
        ReminderSettings {
            enabled: false,
            interval_minutes: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerState {
    pub last_log_time: Option<i64>, // Unix timestamp
    pub accumulated_seconds: u32,
    pub last_ticket: Option<String>,
    pub last_log_date: Option<String>, // YYYY-MM-DD for daily reset
}

impl Default for TimerState {
    fn default() -> Self {
        TimerState {
            last_log_time: None,
            accumulated_seconds: 0,
            last_ticket: None,
            last_log_date: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    pub name: String,
    pub email: String,
    pub api_key: String,
    pub jira_url: String,
    pub custom_meetings: Vec<Project>,
    pub reminder_settings: ReminderSettings,
    pub timer_state: TimerState,
}

impl Preferences {
    pub fn new() -> Self {
        Preferences {
            name: String::new(),
            email: String::new(),
            api_key: String::new(),
            jira_url: String::new(),
            custom_meetings: seed_meeting_tickets(),
            reminder_settings: ReminderSettings::default(),
            timer_state: TimerState::default(),
        }
    }

    pub fn load() -> Result<Self, Error> {
        let input = std::fs::read_to_string(
            dirs::home_dir()
                .unwrap_or_default()
                .join(".config")
                .join(PREF_FILENAME),
        )?;

        let prefs = match serde_json::from_str::<Preferences>(&input) {
            Ok(p) => p,
            Err(_) => {
                let mut prefs = Preferences::new();
                for line in input.lines() {
                    match line.split_once('=').unwrap() {
                        ("NAME", name) => {
                            prefs.set_name(name);
                        }
                        ("API_KEY", api_key) => {
                            prefs.set_api_key(api_key);
                        }
                        _ => {}
                    }
                }
                Self::backup().ok();
                prefs.save()?;
                prefs
            }
        };

        Ok(prefs)
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn set_api_key(&mut self, api_key: &str) -> &mut Self {
        self.api_key = api_key.to_string();
        self
    }

    pub fn set_email(&mut self, email: &str) -> &mut Self {
        self.email = email.to_string();
        self
    }

    pub fn set_jira_url(&mut self, jira_url: &str) -> &mut Self {
        self.jira_url = jira_url.to_string();
        self
    }

    pub fn save(&self) -> Result<(), Error> {
        let path = dirs::home_dir().unwrap_or_default().join(".config");
        std::fs::create_dir_all(&path)?;
        std::fs::write(
            path.join(PREF_FILENAME),
            serde_json::to_string_pretty(self)?,
        )?;

        Ok(())
    }

    fn backup() -> Result<(), Error> {
        let path = dirs::home_dir().unwrap_or_default().join(".config");
        std::fs::copy(
            path.join(PREF_FILENAME),
            path.join(format!("{PREF_FILENAME}.bak")),
        )?;

        Ok(())
    }
    
    // Helper to check if we should reset accumulated time
    pub fn should_reset_timer(&self) -> bool {
        let now = OffsetDateTime::now_utc();
        let today = format!("{:04}-{:02}-{:02}", now.year(), now.month() as u8, now.day());
        
        // Reset if it's a new day
        if let Some(last_date) = &self.timer_state.last_log_date {
            if last_date != &today {
                return true;
            }
        }
        
        // Reset if gap is > 12 hours
        if let Some(last_time) = self.timer_state.last_log_time {
            let elapsed = now.unix_timestamp() - last_time;
            if elapsed > 12 * 3600 {
                return true;
            }
        }
        
        false
    }
    
    // Update timer state after logging
    pub fn update_timer_state(&mut self, ticket: &str) {
        let now = OffsetDateTime::now_utc();
        
        self.timer_state.last_log_time = Some(now.unix_timestamp());
        self.timer_state.last_ticket = Some(ticket.to_string());
        self.timer_state.last_log_date = Some(format!("{:04}-{:02}-{:02}", 
            now.year(), now.month() as u8, now.day()));
        self.timer_state.accumulated_seconds = 0; // Reset after logging
    }
    
    // Get elapsed time since last log
    pub fn get_elapsed_seconds(&self) -> u32 {
        if let Some(last_time) = self.timer_state.last_log_time {
            let now = OffsetDateTime::now_utc();
            let elapsed = (now.unix_timestamp() - last_time) as u32;
            elapsed + self.timer_state.accumulated_seconds
        } else {
            0
        }
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Self::new()
    }
}
