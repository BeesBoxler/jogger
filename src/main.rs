pub mod categories;
pub mod components;
pub mod jira;
pub mod preferences;
pub mod time;

use components::create_menu_dialog;
use cursive::{
    theme::{BaseColor::Green, Color::Dark},
    Cursive, CursiveExt,
};
use preferences::Preferences;
use std::{cell::RefCell, rc::Rc};

const WIDTH: usize = 86;

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
    c.add_layer(create_menu_dialog(prefs, WIDTH));

    c.run();
}
