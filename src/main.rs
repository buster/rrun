#![crate_type = "bin"]
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate gtk;
extern crate gdk;
extern crate toml;
extern crate itertools;

use gtk::traits::*;
use std::rc::Rc;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::error::Error;
use std::path::Path;
use std::env;
use itertools::Itertools;
use std::cell::Cell;
use std::sync::{Arc, Mutex};
use gtk::signal::Inhibit;
use gdk::enums::key;
use gdk::enums::modifier_type;
use externalautocompleter::ExternalAutoCompleter;
use autocomplete::AutoCompleter;
mod autocomplete;
mod externalautocompleter;
mod execution;

#[macro_export]
macro_rules! trys {($e: expr) => {match $e {
    Ok (ok) => ok,
    Err (err) => {
        return Err (format! ("{}:{}] {}", file!(), line!(), err));
        }
    }
}
}

#[cfg(feature="search_entry")]
fn get_entry_field() -> gtk::SearchEntry {
    gtk::SearchEntry::new().unwrap()
}

#[cfg(not(feature="search_entry"))]
fn get_entry_field() -> gtk::Entry {
    gtk::Entry::new().unwrap()
}

fn get_config_file() -> Result<File, String> {
    // Create a path to the desired file
    let config_directory = match env::home_dir() {
        Some(dir) => dir.join(Path::new(".config/rrun")),
        None => panic!("Unable to get $HOME")
    };
    if fs::create_dir_all(&config_directory).is_err() {
        panic!("Unable to create config directory {:?}", config_directory);
    };
    let config_path = config_directory.join(Path::new("config.toml"));

    match File::open(&config_path) {
        Err(why) => {
            info!("couldn't open {}: {}", config_path.display(), Error::description(&why));
            println!("Creating initial config file in ~/.config/rrun/config.toml.");
            let mut f = trys!(File::create(&config_path));
            trys!(f.write_all(include_str!("config.toml").as_bytes()));
            trys!(f.flush());
            drop(f);
            Ok(trys!(File::open(&config_path)))
        },
        Ok(file) => Ok(file),
    }
}


fn get_completers(config: &mut File) -> Vec<Box<autocomplete::AutoCompleter>> {
    let mut toml = String::new();
    match config.read_to_string(&mut toml) {
        Err(why) => panic!("couldn't read Configfile ~/.config/rrun/config.toml: {}",
                                                   Error::description(&why)),
        Ok(_) => (),
    }

    let value = toml::Parser::new(&toml).parse().unwrap();
    debug!("TOML is {:?}", value);
    let maybe_completions = value.get("completion").into_iter();
    let completions = maybe_completions.flat_map(|cs| cs.as_slice().unwrap().into_iter());
    let autocompleter_configs = completions.flat_map(|cs| cs.as_table());
     autocompleter_configs.map(|cfg| {
        let command = cfg.get("command").and_then(|c| c.as_str()).map(|c| c.to_string()).unwrap();
        ExternalAutoCompleter::new(command)
    }).collect()
}

#[allow(dead_code)]
fn main() {
    let mut file = get_config_file().unwrap();
    // Read the file contents into a string, returns `io::Result<usize>`
    let autocompleters = get_completers(&mut file);

    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    debug!("Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());
    let last_pressed_key: Rc<Cell<i32>> = Rc::new(Cell::new(0));

    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
    env_logger::init().unwrap();
    let entry = get_entry_field();

    window.set_title("rrun");
    window.set_window_position(gtk::WindowPosition::Center);

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
                    *completions.lock().unwrap() = autocompleters.iter().map(|c| c.complete(text)).fold1(|c1, c2| Box::new(c1.chain(c2))).unwrap();
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
