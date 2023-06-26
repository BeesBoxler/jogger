#![allow(deprecated)]

use serde::{Deserialize, Serialize};

use std::env::home_dir;
use std::io::Error;
use std::{cell::RefCell, rc::Rc};

use crate::meeting_types::{seed_meeting_tickets, Project};

const PREF_FILENAME: &str = "jogger.conf";

pub type PrefRef = Rc<RefCell<Preferences>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    pub name: String,
    pub api_key: String,
    pub personal_distraction: String,
    pub jira_url: String,
    pub custom_meetings: Vec<Project>,
}

impl Preferences {
    pub fn new() -> Self {
        Preferences {
            name: String::new(),
            api_key: String::new(),
            personal_distraction: String::new(),
            jira_url: String::new(),
            custom_meetings: seed_meeting_tickets(),
        }
    }

    pub fn load() -> Result<Self, Error> {
        let input = std::fs::read_to_string(
            home_dir()
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
                        ("PERSONAL_DISTRACTION", personal_distraction) => {
                            prefs.set_personal_distraction(personal_distraction);
                        }
                        ("JIRA_URL", jira_url) => {
                            prefs.set_jira_url(jira_url);
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

    pub fn set_personal_distraction(&mut self, personal_distraction: &str) -> &mut Self {
        self.personal_distraction = personal_distraction.to_string();
        self
    }

    pub fn set_api_key(&mut self, api_key: &str) -> &mut Self {
        self.api_key = api_key.to_string();
        self
    }

    pub fn set_jira_url(&mut self, jira_url: &str) -> &mut Self {
        self.jira_url = jira_url.to_string();
        self
    }

    pub fn save(&self) -> Result<(), Error> {
        let path = home_dir().unwrap_or_default().join(".config");
        std::fs::create_dir_all(&path)?;
        std::fs::write(
            path.join(PREF_FILENAME),
            serde_json::to_string_pretty(self)?,
        )?;

        Ok(())
    }

    fn backup() -> Result<(), Error> {
        let path = home_dir().unwrap_or_default().join(".config");
        std::fs::copy(
            path.join(PREF_FILENAME),
            path.join(format!("{PREF_FILENAME}.bak")),
        )?;

        Ok(())
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Self::new()
    }
}
