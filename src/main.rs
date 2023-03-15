mod categories;
mod jira;
mod preferences;
mod time;

use categories::categories;
use cursive::theme::BaseColor::Green;
use cursive::theme::Color::Dark;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, EditView, LinearLayout, Panel, SelectView, TextView, ViewRef};
use cursive::{Cursive, CursiveExt};
use jira::{submit_timelog, TimeLog};
use preferences::Preferences;
use std::cell::RefCell;
use std::rc::Rc;

const WIDTH: usize = 86;

type PrefRef = Rc<RefCell<Preferences>>;

fn main() {
    let prefs = Rc::new(RefCell::new(
        Preferences::load().unwrap_or(Preferences::new()),
    ));

    let mut c = Cursive::new();
    c.add_global_callback('q', |c| match c.pop_layer() {
        Some(_) => c.noop(),
        None => c.quit(),
    });

    c.update_theme(|theme| theme.palette.set_color("Background", Dark(Green)));
    c.set_window_title("Jogger");
    c.add_layer(create_menu_dialog(prefs).fixed_width(WIDTH));

    c.run();
}

fn create_menu_dialog(prefs: PrefRef) -> Dialog {
    let menu = SelectView::new()
        .item("Log Time to a Ticket", 1)
        .item("Log Personal Distraction", 2)
        .item("Setup", 3)
        .item("Quit", 4)
        .on_submit(move |c, item| {
            let prefs = prefs.clone();
            match item {
                2 => c.add_layer(
                    create_time_log_dialog(
                        prefs.clone(),
                        Some("Log Personal Distraction"),
                        prefs.borrow().personal_distraction.clone(),
                    )
                    .fixed_width(WIDTH),
                ),
                3 => c.add_layer(create_setup_dialog(prefs).fixed_width(WIDTH)),
                4 => c.quit(),
                _ => c.add_layer(
                    Dialog::around(TextView::new("This function has not yet been implemented."))
                        .button("Okay", |c| {
                            c.pop_layer();
                        }),
                ),
            }
        });

    Dialog::around(menu).title("Jogger")
}

fn create_time_log_dialog(prefs: PrefRef, title: Option<&str>, issue: String) -> Dialog {
    let mut categories_view = SelectView::new();
    for category in categories().keys().into_iter() {
        categories_view.add_item_str(*category);
    }
    categories_view.set_on_select(|c, item| {
        let mut action_view = c.find_name::<SelectView>("action").unwrap();
        action_view.clear();
        let actions: Vec<&str> = categories()
            .get(item.as_str())
            .unwrap()
            .iter()
            .map(|v| *v)
            .collect();
        action_view.add_all_str(actions);
    });

    let actions_view = SelectView::new().item_str("Please Select a Category");

    let view = LinearLayout::vertical()
        .child(
            LinearLayout::horizontal()
                .child(Panel::new(categories_view.with_name("category")).fixed_width(WIDTH / 2))
                .child(Panel::new(actions_view.with_name("action")).fixed_width(WIDTH / 2)),
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
}

fn create_setup_dialog(prefs: PrefRef) -> Dialog {
    let prefs = prefs.clone();
    let layout = LinearLayout::vertical()
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("Your Name: "))
                .child(
                    EditView::new()
                        .content(&prefs.borrow().name)
                        .with_name("name")
                        .full_width(),
                ),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("API Key: "))
                .child(
                    EditView::new()
                        .content(&prefs.borrow().api_key)
                        .with_name("api_key")
                        .full_width(),
                ),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("Personal Distraction Ticket: "))
                .child(
                    EditView::new()
                        .content(&prefs.borrow().personal_distraction)
                        .with_name("personal_distraction")
                        .full_width(),
                ),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("Jira API URL: "))
                .child(
                    EditView::new()
                        .content(&prefs.borrow().jira_url)
                        .with_name("jira_url")
                        .full_width(),
                ),
        );

    Dialog::around(layout)
        .button("Save", move |c| {
            let prefs = prefs.clone();
            let name = &*(c.find_name("name").unwrap() as ViewRef<EditView>).get_content();
            let api_key = &*(c.find_name("api_key").unwrap() as ViewRef<EditView>).get_content();
            let personal_distraction =
                &*(c.find_name("personal_distraction").unwrap() as ViewRef<EditView>).get_content();
            let jira_url = &*(c.find_name("jira_url").unwrap() as ViewRef<EditView>).get_content();

            prefs
                .borrow_mut()
                .set_name(name)
                .set_api_key(api_key)
                .set_personal_distraction(personal_distraction)
                .set_jira_url(jira_url);

            match prefs.clone().borrow().save() {
                Ok(_) => {
                    c.pop_layer();
                }
                Err(err) => c.add_layer(Dialog::around(TextView::new(format!(
                    "An error occured: {}",
                    err
                )))),
            }
        })
        .button("Cancel", |c| {
            c.pop_layer();
        })
        .title("Setup")
        .padding_top(1)
}
