#![crate_type = "bin"]
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate gtk;
extern crate gdk;
extern crate toml;
extern crate itertools;
extern crate regex;

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
use gtk::widgets;
use gtk::signal::Inhibit;
use gdk::enums::key;
use gdk::enums::modifier_type;
use autocomplete::Completion;
use engine::DefaultEngine;
use engine::Engine;


#[macro_export]
macro_rules! trys {($e: expr) => {match $e {
    Ok (ok) => ok,
    Err (err) => {
        return Err (format! ("{}:{}] {}", file!(), line!(), err));
        }
    }
}
}
mod engine;
mod autocomplete;
mod externalautocompleter;
mod runner;
mod externalrunner;
mod execution;


#[cfg(feature="search_entry")]
fn get_entry_field() -> gtk::SearchEntry {
    gtk::SearchEntry::new().unwrap_or_else(|| panic!("Unable to instantiate GTK::SearchEntry!"))
}

#[cfg(not(feature="search_entry"))]
fn get_entry_field() -> gtk::Entry {
    gtk::Entry::new().unwrap_or_else(|| panic!("Unable to instantiate GTK::Entry!"))
}

fn get_config_file() -> Result<File, String> {
    // Create a path to the desired file
    let config_directory = match env::home_dir() {
        Some(dir) => dir.join(Path::new(".config/rrun")),
        None => panic!("Unable to get $HOME"),
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
        }
        Ok(file) => Ok(file),
    }
}

fn read_config(config_file: &mut File) -> toml::Table {
    let mut toml = String::new();
    match config_file.read_to_string(&mut toml) {
        Err(why) => panic!("couldn't read Configfile ~/.config/rrun/config.toml: {}", Error::description(&why)),
        Ok(_) => (),
    }

    let config = toml::Parser::new(&toml).parse().unwrap_or_else(|| panic!("Unable to parse config file TOML!"));
    debug!("config.toml contains the following configuration\n{:?}", config);
    config
}

#[allow(dead_code)]
fn main() {
    let mut file = get_config_file().unwrap_or_else(|x| panic!("Unable to read configuration! {}", x));
    let config = read_config(&mut file);
    let engine = DefaultEngine::new(&config);

    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    debug!("Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());
    let glade_src = include_str!("rrun.glade");
    let builder = widgets::Builder::new_from_string(glade_src).unwrap();
    let (window, entry) = unsafe {
        let window: gtk::Window = builder.get_object("rrun").unwrap();
        let container: gtk::widgets::Box = builder.get_object("container").unwrap();
        let entry = get_entry_field();
        container.add(&entry);
        window.connect_delete_event(|_, _| {
           gtk::main_quit();
           Inhibit(false)
        });
        window.set_border_width(0);
        window.set_decorated(false);
        window.show_all();
        (window, entry)
    };

    let last_pressed_key: Rc<Cell<i32>> = Rc::new(Cell::new(0));

    env_logger::init().unwrap_or_else(|x| panic!("Error initializing logger: {}", x));

    let completion_iterator: Arc<Mutex<Box<Iterator<Item = Completion>>>> =
        Arc::new(Mutex::new(Box::new(vec![].into_iter())));
    let current_completion: Arc<Mutex<Box<Option<Completion>>>> = Arc::new(Mutex::new(Box::new(None)));
    window.connect_key_press_event(move |_, key| {
        let keyval = key.keyval as i32;
        let keystate = (*key).state;
        debug!("key pressed: {}", keyval);
        match keyval {
            key::Escape => gtk::main_quit(),
            key::Return => {
                debug!("keystate: {:?}", keystate);
                debug!("Controlmask == {:?}", modifier_type::ControlMask);
                let query = entry.get_text().unwrap_or_else(|| panic!("Unable to get string from Entry widget!"));
                let comp = *current_completion.lock()
                                              .unwrap_or_else(|x| panic!("Unable to lock current_completion {:?}", x))
                                              .clone();
                let the_completion = match comp {
                    Some(completion) => completion,
                    None => engine.get_completions(&query).next().unwrap().to_owned(),
                };

                if keystate.intersects(modifier_type::ControlMask) {
                    let output = engine.run_completion(&the_completion, false)
                                       .unwrap_or_else(|x| panic!("Error while executing the command {:?}", x));
                    debug!("ctrl pressed!");
                    if output.len() > 0 {
                        entry.set_text(output.trim());
                        entry.set_position(-1);
                    }
                } else {
                    let _ = engine.run_completion(&the_completion, true)
                                  .unwrap_or_else(|x| panic!("Error while executing {:?} in the background!", x));
                    gtk::main_quit();
                }

            }
            key::Tab => {
                if last_pressed_key.get() != key::Tab {
                    let query = &entry.get_text().unwrap_or_else(|| panic!("Unable to get string from Entry widget!"));
                    let current_completions = engine.get_completions(query);
                    *completion_iterator.lock().unwrap() = current_completions;
                }
                let new_completion = completion_iterator.lock().unwrap().next();
                debug!("new_completion: {:?}", new_completion);

                if new_completion.is_some() {
                    *current_completion.lock().unwrap() = Box::new(new_completion.clone());
                    entry.set_text(new_completion.unwrap().text.trim());
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
