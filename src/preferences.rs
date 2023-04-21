#![allow(deprecated)]

use std::env::home_dir;
use std::io::Error;
use std::{cell::RefCell, rc::Rc};

const PREF_FILENAME: &str = "jogger.conf";

pub type PrefRef = Rc<RefCell<Preferences>>;

#[derive(Debug, Clone)]
pub struct Preferences {
    pub name: String,
    pub api_key: String,
    pub personal_distraction: String,
    pub jira_url: String,
}

impl Preferences {
    pub fn new() -> Self {
        Preferences {
            name: String::new(),
            api_key: String::new(),
            personal_distraction: String::new(),
            jira_url: String::new(),
        }
    }

    pub fn load() -> Result<Self, Error> {
        let input = std::fs::read_to_string(
            home_dir()
                .unwrap_or_default()
                .join(".config")
                .join(PREF_FILENAME),
        )?;
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
        std::fs::write(path.join(PREF_FILENAME), format!("{self}").as_bytes())?;

        Ok(())
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Preferences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NAME={}", self.name)?;
        writeln!(f, "API_KEY={}", self.api_key)?;
        writeln!(f, "PERSONAL_DISTRACTION={}", self.personal_distraction)?;
        writeln!(f, "JIRA_URL={}", self.jira_url)
    }
}
