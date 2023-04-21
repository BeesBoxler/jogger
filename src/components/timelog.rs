use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, Panel, SelectView, TextView, ViewRef},
    Cursive, View,
};

use crate::{
    categories::categories,
    jira::{submit_timelog, TimeLog},
    preferences::PrefRef,
    time,
};

fn submit(c: &mut Cursive, prefs: PrefRef, width: usize) {
    let issue = c.find_name::<EditView>("issue").unwrap().get_content();
    c.pop_layer();
    c.add_layer(create_time_log_dialog(
        prefs.clone(),
        Some(&format!("Logging Time for {issue}")),
        issue.to_string(),
        width.clone(),
    ));
}

pub fn create_issue_input_dialog(prefs: PrefRef, width: usize) -> Box<dyn View> {
    let p = prefs.clone();
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

pub fn create_time_log_dialog(
    prefs: PrefRef,
    title: Option<&str>,
    issue: String,
    width: usize,
) -> Box<dyn View> {
    let mut categories_view = SelectView::new();
    categories()
        .iter()
        .enumerate()
        .for_each(|(i, c)| categories_view.add_item(c.0, i));

    categories_view.set_on_select(|c, item| {
        let mut action_view = c.find_name::<SelectView>("action").unwrap();
        action_view.clear();
        let actions: Vec<&str> = categories()[*item].1.iter().map(|v| *v).collect();
        action_view.add_all_str(actions);
    });

    let actions_view = SelectView::new().item_str("Please Select a Category");

    let view = LinearLayout::vertical()
        .child(
            LinearLayout::horizontal()
                .child(
                    Panel::new(categories_view.with_name("category"))
                        .fixed_width(width / 2)
                        .fixed_height(15),
                )
                .child(
                    Panel::new(actions_view.with_name("action"))
                        .fixed_width(width / 2)
                        .fixed_height(15),
                ),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("Time: "))
                .child(EditView::new().with_name("time").full_width()),
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
                let category = c
                    .find_name::<SelectView>("category")
                    .unwrap()
                    .selection()
                    .unwrap_or_default();
                let action = c
                    .find_name::<SelectView>("action")
                    .unwrap()
                    .selection()
                    .unwrap_or_default();
                let comment = c.find_name::<EditView>("comment").unwrap().get_content();

                let body = format!("{category}:{action}::{comment}");

                let time_input: ViewRef<EditView> = c.find_name("time").unwrap();
                match time::string_to_seconds(time_input.get_content().as_str()) {
                    Ok(time) => {
                        c.add_layer(Dialog::around(TextView::new("Uploading...")));
                        let prefs = prefs.borrow();

                        match submit_timelog(&TimeLog {
                            time_spent_seconds: time,
                            comment: body,
                            ticket_number: issue.to_string(),
                            url: prefs.jira_url.to_string(),
                            api_key: prefs.api_key.to_string(),
                        }) {
                            Ok(_) => c.add_layer(
                                Dialog::around(TextView::new(format!("Successful"))).button(
                                    "Okay",
                                    |c| {
                                        c.pop_layer();
                                        c.pop_layer();
                                        c.pop_layer();
                                    },
                                ),
                            ),
                            Err(err) => c.add_layer(
                                Dialog::around(TextView::new(format!("ERROR: {}", err.mgs())))
                                    .button("Okay", |c| {
                                        c.pop_layer();
                                        c.pop_layer();
                                    }),
                            ),
                        };
                    }
                    Err(err) => c.add_layer(
                        Dialog::around(TextView::new(format!("ERROR: {}", err.msg()))).button(
                            "Okay",
                            |c| {
                                c.pop_layer();
                            },
                        ),
                    ),
                };
            })
            .button("Cancel", |c| {
                c.pop_layer();
            })
            .fixed_width(width),
    )
}
