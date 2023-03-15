mod categories;
mod preferences;
mod time;

use categories::categories;
use cursive::theme::BaseColor::Green;
use cursive::theme::Color::Dark;
use std::cell::RefCell;
use std::rc::Rc;

use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, EditView, LinearLayout, Panel, SelectView, TextView, ViewRef};
use cursive::{Cursive, CursiveExt};
use preferences::Preferences;

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
                    create_time_log_dialog(prefs, Some("Log Personal Distraction"))
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

fn create_time_log_dialog(_: PrefRef, title: Option<&str>) -> Dialog {
    let mut categories_view = SelectView::new();
    for category in categories().keys().into_iter() {
        categories_view.add_item_str(*category);
    }
    categories_view.set_on_select(|c, item| {
        let mut action_view = c.find_name("actions").unwrap() as ViewRef<SelectView>;
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
                .child(Panel::new(actions_view.with_name("actions")).fixed_width(WIDTH / 2)),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("Time: "))
                .child(EditView::new().with_name("time").full_width()),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("Comment: "))
                .child(EditView::new().full_width().with_name("comment")),
        );

    Dialog::around(view)
        .title(title.unwrap_or("Create Time Log"))
        .button("Submit", |c| {
            let time_input: ViewRef<EditView> = c.find_name("time").unwrap();
            match time::string_to_seconds(time_input.get_content().as_str()) {
                Ok(time) => c.add_layer(Dialog::around(TextView::new(format!("Logging {time} seconds"))).button(
                    "Okay",
                    |c| {
                        c.pop_layer();
                        c.pop_layer();
                    },
                )),
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
        );

    Dialog::around(layout)
        .button("Save", move |c| {
            let prefs = prefs.clone();
            let name = &*(c.find_name("name").unwrap() as ViewRef<EditView>).get_content();
            let api_key = &*(c.find_name("api_key").unwrap() as ViewRef<EditView>).get_content();
            let personal_distraction =
                &*(c.find_name("personal_distraction").unwrap() as ViewRef<EditView>).get_content();

            prefs
                .borrow_mut()
                .set_name(name)
                .set_api_key(api_key)
                .set_personal_distraction(personal_distraction);

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
