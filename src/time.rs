use regex::{Match, Regex};

pub struct Error(String);
impl Error {
    pub fn msg(&self) -> String {
        self.0.clone()
    }
}

const MINUTE: f64 = 60f64;
const HOUR: f64 = 60f64 * 60f64;

pub fn string_to_seconds(time: &str) -> Result<usize, Error> {
    let r = Regex::new(r#"(?:(\d+)m)|(?:(\d+)h)(?:(\d*)m?)?|(?:(\d+\.?\d*)h?)"#).unwrap();
    let result = r.captures(time);

    let mut seconds = 0;

    match result {
        Some(result) => {
            seconds += (parse_match(result.get(1)) * MINUTE).floor() as usize;
            seconds += (parse_match(result.get(2)) * HOUR).floor() as usize;
            seconds += (parse_match(result.get(3)) * MINUTE).floor() as usize;
            seconds += (parse_match(result.get(4)) * HOUR).floor() as usize;
        }
        None => {
            return Err(Error(format!(
                "Time could not be parsed from string `{time}`"
            )))
        }
    }

    Ok(seconds)
}

fn parse_match(string: Option<Match>) -> f64 {
    string
        .and_then(|value| value.as_str().parse().ok())
        .unwrap_or_default()
}

#[cfg(test)]
mod test {
    use super::string_to_seconds;

    #[test]
    fn explicit_hour_with_implicit_minute() {
        let seconds = string_to_seconds("1h30").ok().unwrap();
        assert_eq!(seconds, 5400);
    }

    #[test]
    fn no_hour_explicit_minute() {
        let seconds = string_to_seconds("15m").ok().unwrap();
        assert_eq!(seconds, 900)
    }

    #[test]
    fn decimal_with_explicit_hour() {
        let seconds = string_to_seconds("1.5h").ok().unwrap();
        assert_eq!(seconds, 5400)
    }

    #[test]
    fn implicit_hour() {
        let seconds = string_to_seconds("1").ok().unwrap();
        assert_eq!(seconds, 3600)
    }

    #[test]
    fn explicit_hour_explicit_minute() {
        let seconds = string_to_seconds("1h30m").ok().unwrap();
        assert_eq!(seconds, 5400)
    }

    #[test]
    fn decimal_with_implicit_hour() {
        let seconds = string_to_seconds("1.5").ok().unwrap();
        assert_eq!(seconds, 5400)
    }

    #[test]
    fn explicit_hour_no_minute() {
        let seconds = string_to_seconds("1h").ok().unwrap();
        assert_eq!(seconds, 3600)
    }

    #[test]
    fn non_time_string() {
        let seconds = string_to_seconds("Look alive, sunshine");
        assert!(seconds.is_err())
    }
}
