use b64_rs::encode;
use reqwest::{blocking::Client, StatusCode};
use serde::Serialize;
use time::{macros::format_description, OffsetDateTime};

use crate::preferences::PrefRef;

pub struct Error(String);

impl Error {
    pub fn msg(&self) -> &str {
        &self.0
    }
}

pub struct TimeLog {
    pub time_spent_seconds: usize,
    pub comment: String,
    pub ticket_number: String,
    pub prefs: PrefRef,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorklogPayload {
    time_spent_seconds: usize,
    comment: String,
    started: String,
}

pub fn submit_timelog(log: &TimeLog) -> Result<(), Error> {
    let prefs = log.prefs.borrow();

    let url = format!(
        "{}/rest/api/2/issue/{}/worklog",
        prefs.jira_url.trim_end_matches('/'),
        log.ticket_number
    );

    let started = OffsetDateTime::now_utc()
        .format(format_description!(
            "[year]-[month]-[day]T[hour]:[minute]:[second].000+0000"
        ))
        .map_err(|e| Error(format!("Failed to format timestamp: {}", e)))?;

    let payload = WorklogPayload {
        time_spent_seconds: log.time_spent_seconds,
        comment: log.comment.clone(),
        started,
    };

    let credentials = format!(
        "Basic {}",
        encode(&format!("{}:{}", prefs.email, prefs.api_key))
    );

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Authorization", credentials)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&payload)
        .send()
        .map_err(|e| Error(format!("Network error: {}", e)))?;

    match response.status() {
        StatusCode::OK | StatusCode::CREATED => Ok(()),
        status => {
            let error_body = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            Err(Error(format!("Jira returned {}: {}", status, error_body)))
        }
    }
}
