pub struct CategoryMapping(pub &'static str, pub Vec<&'static str>);

pub fn categories() -> Vec<CategoryMapping> {
    vec![
        CategoryMapping("None", vec![]),
        CategoryMapping("Assistance", vec!["Development", "Testing"]),
        CategoryMapping(
            "Business Analyst",
            vec![
                "Requirements Gathering",
                "Post-development Documentation",
                "Pre-development Documentation",
                "Ticket Creation",
                "UAT",
            ],
        ),
        CategoryMapping(
            "Development",
            vec!["Development on Task", "Technical Investigation"],
        ),
        CategoryMapping(
            "Personal Distraction",
            vec![
                "Mentoring",
                "Live Issue Discussion/Investigating",
                "Adhoc Meeting/Discussion (IT)",
                "Planned Team Meeting",
                "Recruitment (Interviews/Assessments)",
                "Training Session",
                "General Stakeholder Interaction",
                "Project Build Issues Discussion/Investigation",
                "Adhoc Meeting/Discussion (Stakeholder)",
                "Planned 1 to 1 Meeting",
                "Workstation/Peripheral Issues",
                "Fire Alarm/Evacuation",
                "Task Request From Manager",
            ],
        ),
        CategoryMapping(
            "Project Management",
            vec!["Project Planning", "Project Reporting"],
        ),
        CategoryMapping("Spike", vec!["Investigation", "Building Proof of Concept"]),
        CategoryMapping(
            "Team Project Task",
            vec![
                "Peer Review (Code/Test)",
                "Standup",
                "Backlog Refinement",
                "Sprint Planning",
                "Retrospective",
                "Sprint Review Prep",
                "Sprint Review",
                "Deployment Planning",
                "Deployment Execution",
                "Visioning",
                "Documentation",
                "Tech Review Meeting",
            ],
        ),
        CategoryMapping(
            "Testing",
            vec![
                "Test Planning",
                "Test Execution",
                "Regression Testing",
                "Test Creation",
                "Test Maintenance",
            ],
        ),
    ]
}
