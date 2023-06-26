use b64_rs::encode;
use reqwest::{blocking::Client, Method, StatusCode};
use time::{macros::format_description, OffsetDateTime};

use crate::preferences::{PrefRef};

pub struct Error(String);

impl Error {
    pub fn mgs(&self) -> String {
        self.0.clone()
    }
}

pub struct TimeLog {
    pub time_spent_seconds: usize,
    pub comment: String,
    pub ticket_number: String,
    pub prefs: PrefRef,
}

pub fn submit_timelog(log: &TimeLog) -> Result<(), Error> {
    let prefs = log.prefs.borrow();
    
    let jira_url = &prefs.jira_url;
    let email = &prefs.email;
    let api_key = &prefs.api_key;

    let url = format!(
        "{}/rest/api/2/issue/{}/worklog",
        jira_url, log.ticket_number
    );
    let client = Client::new();

    let data = format!(
        r#"{{
            "timeSpentSeconds": {},
            "comment": "{}",
            "started":"{}"
        }}"#,
        log.time_spent_seconds,
        log.comment,
        OffsetDateTime::now_utc()
            .format(format_description!(
                "[year]-[month]-[day]T[hour]:[minute]:[second].000+0000"
            ))
            .unwrap()
    );

    let credentials = format!("Basic {}", encode(&format!("{}:{}", email, api_key)));

    let request = client
        .request(Method::POST, url)
        .header("Authorization", credentials)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(data)
        .build()
        .unwrap();

    match client.execute(request) {
        Ok(response) => match response.status() {
            StatusCode::OK | StatusCode::CREATED => Ok(()),
            result => Err(Error(result.to_string())),
        },
        Err(err) => Err(Error(err.to_string())),
    }
}
