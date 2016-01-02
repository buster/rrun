#![crate_type = "bin"]
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate gtk;
extern crate gdk;

use gtk::traits::*;
use std::rc::Rc;
use std::cell::Cell;
use std::sync::{Arc, Mutex};
use gtk::signal::Inhibit;
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
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    println!("Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());
    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
    env_logger::init().unwrap();
    let entry = get_entry_field();

    window.set_title("rrun");
    window.set_window_position(gtk::WindowPosition::Center);

    let autocompleter = BashAutoCompleter::new();
    let last_pressed_key: Rc<Cell<i32>> = Rc::new(Cell::new(0));

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    window.set_decorated(false);
    window.add(&entry);
    window.set_border_width(0);
    window.show_all();
    let the_completions: Arc<Mutex<Box<Iterator<Item = String>>>> = Arc::new(Mutex::new(Box::new(vec![].into_iter())));
    window.connect_key_press_event(move |_, key| {
        let completions = the_completions.clone();
        let keyval = key.keyval as i32;
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


                } else {
                    execution::execute(cmd, true);
                    gtk::main_quit();
                }

            }
            key::Tab => {
                if last_pressed_key.get() != key::Tab {
                    let text = &entry.get_text().unwrap();
                    *completions.lock().unwrap() = autocompleter.complete(text);
                }
                let new_completion = completions.lock().unwrap().next();

                if new_completion.is_some() {
                    entry.set_text(new_completion.unwrap().trim());
                    entry.set_position(-1);
                    last_pressed_key.set(key::Tab);
                    return Inhibit(true);
                }
            }
            _ => (),
        }
        last_pressed_key.set((*key).keyval as i32);
        Inhibit(false)
    });

    gtk::main();
}
