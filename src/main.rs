#![crate_type = "bin"]
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate gtk;
extern crate gdk;

use std::rc::Rc;
use std::cell::{Cell, RefCell};
use gtk::signal::Inhibit;
use gtk::traits::*;
use gdk::enums::key;
use gdk::enums::modifier_type;
use bashautocompleter::BashAutoCompleter;
use autocomplete::AutoCompleter;
mod autocomplete;
mod bashautocompleter;
mod execution;

#[cfg(feature="search_entry")]
fn get_entry_field() -> gtk::SearchEntry {
    gtk::SearchEntry::new().unwrap()
}

#[cfg(not(feature="search_entry"))]
fn get_entry_field() -> gtk::Entry {
    gtk::Entry::new().unwrap()
}

#[allow(dead_code)]
fn main() {
    env_logger::init().unwrap();
    gtk::init();
    debug!("GTK VERSION: Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());
    let window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();
    let entry = get_entry_field();

    window.set_title("rrun");
    window.set_window_position(gtk::WindowPosition::Center);

    let autocompleter = Rc::new(RefCell::new(BashAutoCompleter::new()));
    let last_pressed_key: Rc<Cell<u32>> = Rc::new(Cell::new(0));

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    window.set_decorated(false);
    window.add(&entry);
    window.set_border_width(0);
    window.show_all();
    window.connect_key_press_event(move |_, key| {

        let keyval: u32 = key.keyval;
        let keystate = (*key).state;
        debug!("key pressed: {}", keyval);
        match keyval {
            key::Escape => gtk::main_quit(),
            key::Return => {
                let cmd = entry.get_text().unwrap();
                debug!("keystate: {:?}", keystate);
                debug!("Controlmask == {:?}", modifier_type::ControlMask);
                if keystate.intersects(modifier_type::ControlMask) {
                    debug!("ctrl pressed!");
                    let output = execution::execute(cmd, false);
                    if output.is_some() {
                        let output = output.unwrap();
                        entry.set_text(output.trim());
                        entry.set_position(-1);
                    }


                }
                else {
                    execution::execute(cmd, true);
                    gtk::main_quit();
                }

            },
            key::Tab => {
                let completion = match last_pressed_key.get() {
                    key::Tab => autocompleter.borrow_mut().complete_next(),
                    _ => autocompleter.borrow_mut().complete_new(&entry.get_text().unwrap())
                };

                if completion.is_some() {
                    entry.set_text(completion.unwrap().trim());
                    entry.set_position(-1);
                    last_pressed_key.set(key::Tab);
                    return Inhibit(true);
                }
            },
            _ => ()
        }
        last_pressed_key.set((*key).keyval);
        return Inhibit(false)
    } );

    gtk::main();
}
