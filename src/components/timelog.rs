use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, DummyView, EditView, LinearLayout, Panel, SelectView, TextView, ViewRef},
    Cursive, View,
};

use crate::{
    jira::{submit_timelog, TimeLog},
    meeting_types::Meeting,
    preferences::PrefRef,
    time,
};

pub fn create_issue_input_dialog(prefs: PrefRef, width: usize) -> Box<dyn View> {
    let p = prefs.clone();

    let submit = |c: &mut Cursive, prefs: PrefRef, width: usize| {
        let issue = c.find_name::<EditView>("issue").unwrap().get_content();
        c.pop_layer();
        c.add_layer(create_looging_dialog(
            prefs,
            Some(&format!("Logging Time for {issue}")),
            issue.to_string(),
            width,
            None,
        ));
    };

    let view = LinearLayout::horizontal()
        .child(TextView::new("Issue Number: "))
        .child(
            EditView::new()
                .on_submit(move |c, _| submit(c, p.clone(), width))
                .with_name("issue")
                .full_width(),
        );

    Box::from(
        Dialog::around(view)
            .button("Continue", move |c| submit(c, prefs.clone(), width))
            .fixed_width(width),
    )
}

pub fn create_meetings_dialog(
    prefs: PrefRef,
    title: Option<&str>,
    issue: String,
    width: usize,
) -> Box<dyn View> {
    let mut projects_list = SelectView::new();
    let projects = prefs.borrow().custom_meetings.clone();
    let height = projects.iter().map(|p| p.meetings.len()).max().unwrap() + 2;

    projects
        .iter()
        .enumerate()
        .for_each(|(i, p)| projects_list.add_item(p.name.clone(), i));

    projects_list.set_on_select(move |c, item| {
        let mut action_view = c.find_name::<SelectView>("meeting").unwrap();
        action_view.clear();
        let meetings: Vec<Meeting> = projects[*item].meetings.clone();
        meetings.iter().for_each(|Meeting(meeting_type, ticket)| {
            action_view.add_item(meeting_type.to_string(), ticket.to_string())
        });
    });

    let meetings_list = SelectView::new().item_str("Please Select a Project");

    let select_meeting = Box::from(
        LinearLayout::horizontal()
            .child(
                Panel::new(projects_list.with_name("project"))
                    .fixed_width(width / 2)
                    .fixed_height(height),
            )
            .child(
                Panel::new(meetings_list.with_name("meeting"))
                    .fixed_width(width / 2)
                    .fixed_height(height),
            ),
    );

    create_looging_dialog(prefs, title, issue, width, Some(select_meeting))
}

fn create_looging_dialog(
    prefs: PrefRef,
    title: Option<&str>,
    issue: String,
    width: usize,
    child: Option<Box<dyn View>>,
) -> Box<dyn View> {
    let i = issue.clone();
    let p = prefs.clone();

    let child = child.unwrap_or(Box::from(DummyView));

    let view = LinearLayout::vertical()
        .child(child)
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("Time: "))
                .child(
                    EditView::new()
                        .on_submit(move |c, _| submit_time_log(c, p.clone(), i.clone()))
                        .with_name("time")
                        .full_width(),
                ),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("Comment: "))
                .child(EditView::new().with_name("comment").full_width()),
        );
    Box::from(
        Dialog::around(view)
            .title(title.unwrap_or("Create Time Log"))
            .button("Submit", move |c| {
                submit_time_log(c, prefs.clone(), issue.clone())
            })
            .button("Cancel", |c| {
                c.pop_layer();
            })
            .fixed_width(width),
    )
}

fn submit_time_log(c: &mut Cursive, prefs: PrefRef, issue: String) {
    let comment = c
        .find_name::<EditView>("comment")
        .unwrap()
        .get_content()
        .to_string();

    let issue = c
        .find_name::<SelectView>("meeting")
        .and_then(|view| view.selection().map(|s| s.to_string()))
        .unwrap_or(issue);

    let time_input: ViewRef<EditView> = c.find_name("time").unwrap();
    match time::string_to_seconds(time_input.get_content().as_str()) {
        Ok(time) => {
            c.add_layer(Dialog::around(TextView::new("Uploading...")));
            let prefs = prefs.borrow();

            match submit_timelog(&TimeLog {
                time_spent_seconds: time,
                comment,
                ticket_number: issue,
                url: prefs.jira_url.to_string(),
                api_key: prefs.api_key.to_string(),
            }) {
                Ok(_) => c.add_layer(
                    Dialog::around(TextView::new("Successful".to_string())).button("Okay", |c| {
                        c.pop_layer();
                        c.pop_layer();
                        c.pop_layer();
                    }),
                ),
                Err(err) => c.add_layer(
                    Dialog::around(TextView::new(format!("ERROR: {}", err.mgs()))).button(
                        "Okay",
                        |c| {
                            c.pop_layer();
                            c.pop_layer();
                        },
                    ),
                ),
            };
        }
        Err(err) => c.add_layer(
            Dialog::around(TextView::new(format!("ERROR: {}", err.msg()))).button("Okay", |c| {
                c.pop_layer();
            }),
        ),
    };
}
