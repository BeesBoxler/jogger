use crate::preferences::PrefRef;
use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, TextView, ViewRef},
    View,
};

pub fn create_setup_dialog(prefs: PrefRef, width: usize) -> Box<dyn View> {
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

    Box::from(
        Dialog::around(layout)
            .button("Save", move |c| {
                let prefs = prefs.clone();
                let name = &*(c.find_name("name").unwrap() as ViewRef<EditView>).get_content();
                let api_key =
                    &*(c.find_name("api_key").unwrap() as ViewRef<EditView>).get_content();
                let personal_distraction = &*(c.find_name("personal_distraction").unwrap()
                    as ViewRef<EditView>)
                    .get_content();
                let jira_url =
                    &*(c.find_name("jira_url").unwrap() as ViewRef<EditView>).get_content();

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
            .fixed_width(width),
    )
}
