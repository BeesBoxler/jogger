use crate::components::{create_issue_input_dialog, create_setup_dialog, create_time_log_dialog};
use crate::preferences::PrefRef;
use cursive::view::Resizable;
use cursive::views::{Dialog, SelectView, TextView};
use cursive::View;

pub fn create_menu_dialog(prefs: PrefRef, width: usize) -> Box<dyn View> {
    let menu = SelectView::new()
        .item("Log Time to a Ticket", 1)
        .item("Log Personal Distraction", 2)
        .item("Setup", 3)
        .item("Quit", 4)
        .on_submit(move |c, item| {
            let prefs = prefs.clone();
            match item {
                1 => c.add_layer(create_issue_input_dialog(prefs, width)),
                2 => c.add_layer(create_time_log_dialog(
                    prefs.clone(),
                    Some("Log Personal Distraction"),
                    prefs.borrow().personal_distraction.clone(),
                    width,
                )),
                3 => c.add_layer(create_setup_dialog(prefs, width)),
                4 => c.quit(),
                _ => c.add_layer(
                    Dialog::around(TextView::new("This function has not yet been implemented."))
                        .button("Okay", |c| {
                            c.pop_layer();
                        }),
                ),
            }
        });

    Box::from(Dialog::around(menu).title("Jogger").fixed_width(width))
}
