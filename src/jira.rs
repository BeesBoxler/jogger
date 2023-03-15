use reqwest::{blocking::Client, Method, StatusCode};
use time::{macros::format_description, OffsetDateTime};

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
    pub url: String,
    pub api_key: String,
}

pub fn submit_timelog(log: &TimeLog) -> Result<(), Error> {
    let url = format!("{}/rest/api/2/issue/{}/worklog", log.url, log.ticket_number);
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

    let request = client
        .request(Method::POST, url)
        .bearer_auth(&log.api_key)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(data)
        .build()
        .unwrap();

    return match client.execute(request) {
        Ok(response) => match response.status() {
            StatusCode::OK | StatusCode::CREATED => Ok(()),
            result => Err(Error(result.to_string())),
        },
        Err(err) => Err(Error(err.to_string())),
    };
}
