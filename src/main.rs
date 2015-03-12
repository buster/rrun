#![feature(libc)]
#![crate_type = "bin"]
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rgtk;
extern crate libc;

use rgtk::*;
use rgtk::gtk::signals::{DeleteEvent,KeyPressEvent};
use rgtk::gdk::key;
use rgtk::gdk::enums::modifier_type;
use autocomplete::{BashAutoCompleter, AutoCompleter};
mod autocomplete;
mod execution;


fn main() {
    env_logger::init().unwrap();
    gtk::init();
    debug!("GTK VERSION: Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());
    let mut window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();
    let mut entry = gtk::SearchEntry::new().unwrap();
    window.set_title("rrun");
    window.set_window_position(gtk::WindowPosition::Center);

    let mut autocompleter: BashAutoCompleter = AutoCompleter::new();
    let mut last_pressed_key: u32 = 0;

    Connect::connect(&window, KeyPressEvent::new(&mut |key| {
        let keyval = unsafe { (*key).keyval };
        let keystate = unsafe { (*key).state };
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
                        entry.set_text(output.trim().to_string());
                        entry.set_position(-1);
                    }


                }
                else {
                    execution::execute(cmd, true);
                    gtk::main_quit();
                }

            },
            key::Tab => {
                let completion = match last_pressed_key {
                    key::Tab => autocompleter.complete_next(),
                    _ => autocompleter.complete_new(&entry.get_text().unwrap())
                };

                if completion.is_some() {
                    entry.set_text(completion.unwrap().trim().to_string());
                    entry.set_position(-1);
                    last_pressed_key = key::Tab;
                    return true;
                }
            },
            _ => ()
        }
        last_pressed_key = unsafe { (*key).keyval };
        return false
    }));


    Connect::connect(&window, DeleteEvent::new(&mut |_|{
        gtk::main_quit();
        true
    }));

    window.set_decorated(false);
    window.add(&entry);
    window.set_border_width(0);
    window.show_all();
    gtk::main();
}
