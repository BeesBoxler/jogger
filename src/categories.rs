pub fn categories() -> Vec<(&'static str, Vec<&'static str>)> {
    let mut categories = vec![];
    categories.push(("None", vec![]));
    categories.push(("Assistance", vec!["Development", "Testing"]));
    categories.push((
        "Business Analyst",
        vec![
            "Requirements Gathering",
            "Post-development Documentation",
            "Pre-development Documentation",
            "Ticket Creation",
            "UAT",
        ],
    ));
    categories.push((
        "Development",
        vec!["Development on Task", "Technical Investigation"],
    ));
    categories.push((
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
    ));
    categories.push((
        "Project Management",
        vec!["Project Planning", "Project Reporting"],
    ));
    categories.push(("Spike", vec!["Investigation", "Building Proof of Concept"]));
    categories.push((
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
    ));
    categories.push((
        "Testing",
        vec![
            "Test Planning",
            "Test Execution",
            "Regression Testing",
            "Test Creation",
            "Test Maintenance",
        ],
    ));

    categories
}

/*
- Assistance
    - Development
    - Testing
- Business Analyst
    - Requirements Gathering
    - Post-development Documentation
    - Pre-development Documentation
    - Ticket Creation
    - UAT
- Development
    - Development on Task
    - Technical Investigation
- Personal Distraction
    - Mentoring
    - Live Issue Discussion/Investigating
    - Adhoc Meeting/Discussion (IT)
    - Planned Team Meeting
    - Recruitment (Interviews/Assessments)
    - Training Session
    - General Stakeholder Interaction
    - Project Build Issues Discussion/Investigation
    - Adhoc Meeting/Discussion (Stakeholder)
    - Planned 1 to 1 Meeting
    - Workstation/Peripheral Issues
    - Fire Alarm/Evacuation
    - Task Request From Manager
- Project Management
    - Project Planning
    - Project Reporting
- Spike
    - Investigation
    - Building Proof of Concept
- Team Project Task
    - Peer Review (Code/Test)
    - Standup
    - Backlog Refinement
    - Sprint Planning
    - Retrospective
    - Sprint Review Prep
    - Sprint Review
    - Deployment Planning
    - Deployment Execution
    - Visioning
    - Documentation
    - Tech Review Meeting
- Testing
    - Test Planning
    - Test Execution
    - Regression Testing
    - Test Creation
    - Test Maintenance

 */
