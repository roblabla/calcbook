use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("../ui.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: gtk::Window = builder.get_object("mainwindow").unwrap();
    window.show_all();

    gtk::main();
}
