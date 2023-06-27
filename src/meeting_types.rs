use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meeting(pub MeetingType, pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeetingType {
    Billable,
    NonBillable,
    Deployment,
    Mentoring,
    Recruitment,
    PersonalDistraction,
}

impl fmt::Display for MeetingType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub meetings: Vec<Meeting>,
}

impl Project {
    pub fn new(name: &str, meetings: Vec<Meeting>) -> Self {
        Project {
            name: name.to_string(),
            meetings,
        }
    }
}

pub fn seed_meeting_tickets() -> Vec<Project> {
    vec![
        Project::new(
            "PIM",
            vec![
                Meeting(MeetingType::Billable, "PIM-6126".to_string()),
                Meeting(MeetingType::NonBillable, "PTD-1528".to_string()),
                Meeting(MeetingType::Deployment, "PIM-6155".to_string()),
            ],
        ),
        Project::new(
            "PP",
            vec![
                Meeting(MeetingType::Billable, "PP-6349".to_string()),
                Meeting(MeetingType::NonBillable, "PTD-1527".to_string()),
                Meeting(MeetingType::Deployment, "PP-6369".to_string()),
            ],
        ),
        Project::new(
            "PAP",
            vec![Meeting(MeetingType::Billable, "PAP-2728".to_string())],
        ),
        Project::new(
            "BED",
            vec![
                Meeting(MeetingType::Billable, "BED-4443".to_string()),
                Meeting(MeetingType::NonBillable, "PTD-1551".to_string()),
            ],
        ),
        Project::new(
            "PTB",
            vec![Meeting(MeetingType::Billable, "PTB-19990".to_string())],
        ),
    ]
}
