#![feature(globs)]
#![crate_type = "bin"]
#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate rgtk;
extern crate libc;

use rgtk::*;
use rgtk::gtk::signals;
use autocomplete::{BashAutoCompleter, AutoCompleter};
mod autocomplete;
mod execution;


fn main() {
    gtk::init();
    debug!("GTK VERSION: Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());
    let mut window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();
    let mut entry = gtk::SearchEntry::new().unwrap();
    window.set_title("rrun");
    window.set_window_position(gtk::WindowPosition::Center);
    
    let mut autocompleter: BashAutoCompleter = AutoCompleter::new();
    let mut last_pressed_key: u32 = 0;

    window.connect(signals::KeyPressEvent::new(|key|{
        let keyval = unsafe { (*key).keyval };
        let keystate = unsafe { (*key).state };
        debug!("key pressed: {}", keyval);
        match keyval {
            65307 => gtk::main_quit(),
            65293 => {
                let cmd = entry.get_text().unwrap();
                debug!("keystate: {}", keystate);
                if keystate == 4 {
                    debug!("ctrl pressed!");
                    let output = execution::execute(cmd, false);
                    if output.is_some() {
                        let output = output.unwrap();
                        entry.set_text(output.trim().into_string());
                        entry.set_position(-1);
                    }

                }
                else {
                    execution::execute(cmd, true);
                    gtk::main_quit();
                }

            },
            65289 => {
                let mut completion = None;
                // last pressed key was TAB, so we want to get the next completion
                if last_pressed_key == 65289 {
                    completion = autocompleter.complete_next();
                }
                else {
                    completion = autocompleter.complete_new(entry.get_text().unwrap().as_slice());    
                }
                
                if completion.is_some() {
                    entry.set_text(completion.unwrap().trim().into_string());
                    entry.set_position(-1);
                    last_pressed_key = 65289;
                    return true;
                }
            },
            _ => ()
        }
        last_pressed_key = unsafe { (*key).keyval };
        return false
    }));

    window.connect(signals::DeleteEvent::new(|_|{
        gtk::main_quit();
        true
    }));

    window.set_decorated(false);
    window.add(&entry);
    window.set_border_width(0);
    window.show_all();
    gtk::main();
}
