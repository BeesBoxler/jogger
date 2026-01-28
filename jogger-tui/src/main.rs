mod components;

use components::create_menu_dialog;
use cursive::{
    theme::{BaseColor::Green, Color::Dark},
    Cursive, CursiveExt,
};
use jogger_core::Preferences;
use std::{cell::RefCell, rc::Rc};

const WIDTH: usize = 86;

fn main() {
    let prefs = Rc::new(RefCell::new(Preferences::load().unwrap_or_default()));

    let mut c = Cursive::new();
    c.add_global_callback('q', |c| {
        if c.screen().len() <= 1 {
            c.quit();
        } else {
            c.pop_layer();
        }
    });

    c.update_theme(|theme| theme.palette.set_color("Background", Dark(Green)));
    c.set_window_title("Jogger");
    c.add_layer(create_menu_dialog(prefs, WIDTH));

    c.run();
}
