#![crate_type = "bin"]
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate gtk;
extern crate gdk;
extern crate glib;
extern crate toml;
extern crate itertools;
extern crate regex;

use gtk::traits::*;
use std::rc::Rc;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::env;
use itertools::Itertools;
use std::cell::{Cell, RefCell};
use std::str::FromStr;
use gtk::widgets;
use gtk::signal::Inhibit;
use gdk::enums::key;
use gdk::keyval_to_unicode;
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


fn append_text_column(tree: &gtk::TreeView) {
    let column = gtk::TreeViewColumn::new().unwrap();
    let cell = gtk::CellRendererText::new().unwrap();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}
fn get_config_dir() -> PathBuf {
    // Create a path to the desired file
    let config_directory = match env::home_dir() {
        Some(dir) => dir.join(Path::new(".config/rrun")),
        None => panic!("Unable to get $HOME"),
    };
    if fs::create_dir_all(&config_directory).is_err() {
        panic!("Unable to create config directory {:?}", config_directory);
    };
    config_directory
}
fn get_config_file(config_path: &Path) -> Result<File, String> {
    match File::open(&config_path) {
        Err(why) => {
            info!("couldn't open {}: {}", config_path.display(), Error::description(&why));
            println!("Creating initial config file in {:?}.", config_path);
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
    let config_directory = get_config_dir();
    let config_path = config_directory.join(Path::new("config.toml"));
    let mut file = get_config_file(&config_path).unwrap_or_else(|x| panic!("Unable to read configuration! {}", x));
    let config = read_config(&mut file);
    let engine = DefaultEngine::new(&config);

    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    debug!("Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());
    let glade_src = include_str!("rrun.glade");
    let builder = widgets::Builder::new_from_string(glade_src).unwrap();
    let (window, entry, completion_list) = unsafe {
        let window: gtk::Window = builder.get_object("rrun").unwrap();
        let css_path = config_directory.join(Path::new("style.css"));
        let css_result_message = css_path.to_str().map(|p|
            widgets::CssProvider::load_from_path(p).map(|cp| {
                widgets::StyleContext::add_provider_for_screen(&window.get_screen(), &cp, 1);
                format!("Applied CSS stylesheet found in {:?}", p)
            }).unwrap_or_else(|e| format!("Could not load CSS stylesheet in {:?}: {}", p, e))
        ).unwrap_or(format!("No CSS stylesheet found in {:?}", css_path));
        debug!("{}", css_result_message);
        let completion_list: gtk::widgets::TreeView = builder.get_object("completion_view").unwrap();
        let entry: gtk::widgets::SearchEntry = builder.get_object("search_entry").unwrap();
        window.connect_delete_event(|_, _| {
           gtk::main_quit();
           Inhibit(false)
        });
        window.set_border_width(0);
        window.set_decorated(false);
        window.show_all();
        (window, entry, completion_list)
    };
    let column_types = [glib::Type::String];
    let completion_store = gtk::ListStore::new(&column_types).unwrap();
    let completion_model = completion_store.get_model().unwrap();

    completion_list.set_model(&completion_model);
    completion_list.set_headers_visible(false);

    append_text_column(&completion_list);

    let last_pressed_key: Rc<Cell<i32>> = Rc::new(Cell::new(0));

    env_logger::init().unwrap_or_else(|x| panic!("Error initializing logger: {}", x));

    let current_completions: Rc<RefCell<Vec<Completion>>> = Rc::new(RefCell::new(vec![]));
    let selected_completion: Rc<RefCell<Option<Completion>>> = Rc::new(RefCell::new(None));
    let current_and_selected_completions = (current_completions.clone(), selected_completion.clone());
    completion_list.get_selection().unwrap().connect_changed(move |tree_selection| {
        if let Some((completion_model, iter)) = tree_selection.get_selected() {
            if let Some(path) = completion_model.get_path(&iter) {
                let selected_number = usize::from_str(path.to_string().unwrap().trim()).unwrap();
                let (ref current_completions, ref selected_completion) = current_and_selected_completions;
                *selected_completion.borrow_mut() = Some(current_completions.borrow()[selected_number].clone());
            }
        }
    });

    window.connect_key_press_event(move |_, key| {
        let keyval = key.keyval as i32;
        let keystate = (*key).state;
        debug!("key pressed: {}", keyval);
        match keyval {
            key::Escape => gtk::main_quit(),
            key::Return => {
                debug!("keystate: {:?}", keystate);
                debug!("Controlmask == {:?}", modifier_type::ControlMask);
                let ref compls_vec = *current_completions;
                let compls = compls_vec.borrow();

                let the_completion = if let Some(completion) = selected_completion.borrow().clone() {
                    completion
                } else if compls.len() > 0 {
                    compls[0].clone()
                } else {
                    let query = entry.get_text().unwrap_or_else(|| panic!("Unable to get string from Entry widget!"));
                    engine.get_completions(&query).next().unwrap().to_owned()
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
            _ => {
                let mut query = entry.get_text().unwrap_or_else(|| panic!("Unable to get string from Entry widget!")).clone();
                if let Some(current_char) = keyval_to_unicode(key.keyval) {
                    query.push(current_char);
                    let completions = engine.get_completions(query.trim()).collect_vec();
                    completion_store.clear();
                    //debug!("Found {:?} completions", completions.len());
                    for (i, cmpl) in completions.iter().enumerate().take(9) {
                        let iter = completion_store.append();
                        completion_store.set_string(&iter, 0, format!("{}. {}", i + 1, cmpl.text).trim());
                    }
                    *current_completions.borrow_mut() = completions;
                }
            },
        }
        last_pressed_key.set((*key).keyval as i32);
        Inhibit(false)
    });

    gtk::main();
}
