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
#[macro_use]
extern crate clap;

use gtk::prelude::*;
use gtk::{TreePath, StyleContext, CellRendererText, Builder, CssProvider, SearchEntry, ListStore, TreeView,
          TreeViewColumn, Window, WindowExt};

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
use gdk::enums::key;
use gdk::keyval_to_unicode;
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


fn append_text_column(tree: &TreeView) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

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

fn get_or_create(file_path: &Path, initial_content: &str) -> Result<File, String> {
    match File::open(&file_path) {
        Err(why) => {
            info!("couldn't open {}: {}", file_path.display(), Error::description(&why));
            println!("Initializing {:?} with default content. Edit it to your liking :D", file_path);
            let mut f = trys!(File::create(&file_path));
            trys!(f.write_all(initial_content.as_bytes()));
            trys!(f.flush());
            drop(f);
            Ok(trys!(File::open(&file_path)))
        }
        Ok(file) => Ok(file),
    }
}

fn read_config(config_file: &mut File) -> toml::value::Table {
    let mut toml = String::new();
    match config_file.read_to_string(&mut toml) {
        Err(why) => panic!("couldn't read Configfile ~/.config/rrun/config.toml: {}", Error::description(&why)),
        Ok(_) => (),
    }

    let config = toml::from_str(&toml).unwrap_or_else(|x| panic!("Unable to parse config file TOML! {}", x));
    debug!("config.toml contains the following configuration\n{:?}", config);
    config
}

fn create_builder_from_file(mut file: &File) -> Builder {
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read file {:?}: {}", file, Error::description(&why)),
        Ok(_) => (),
    }
    Builder::new_from_string(&content)
}

#[allow(dead_code)]
fn main() {
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    let matches = clap_app!(rrun =>
        (version: VERSION.unwrap_or("unknown"))
        (about: "Extensible Quick-Launcher")
        (@subcommand completions =>
            (about: "List completions matching the given query")
            (@arg type: "The type of the completion in config.toml (e.g. url, command, ...)")
            (@arg query: "The query for which to list completions")
        )
    )
        .get_matches();

    let config_directory = get_config_dir();
    let config_path = config_directory.join(Path::new("config.toml"));
    let mut file = get_or_create(&config_path, include_str!("config.toml"))
        .unwrap_or_else(|x| panic!("Unable to read configuration! {}", x));
    let config = read_config(&mut file);
    let engine = DefaultEngine::new(&config);

    if let Some(ref matches) = matches.subcommand_matches("completions") {
        if let (Some(tpe), Some(query)) = (matches.value_of("type"), matches.value_of("query")) {
            let completions = engine.get_completions(&query).filter(|c| c.tpe == tpe);
            let completion_tsv = completions.map(|c| format!("{}\t{}", c.id, c.text)).join("\n");
            println!("{}", completion_tsv);
        } else {
            panic!("If you call completions, you need to provide type and query\nIf unsure, run 'rrun completions \
                    --help'")
        }
    } else {
        run_ui(&config_directory, engine);
    }
}

fn run_ui(config_directory: &Path, engine: DefaultEngine) {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    debug!("Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());
    let ui_path = config_directory.join(Path::new("rrun.glade"));
    let ui_file = get_or_create(&ui_path, include_str!("rrun.glade"))
        .unwrap_or_else(|x| panic!("Unable to read configuration! {}", x));
    let builder = create_builder_from_file(&ui_file);
    let window: Window = builder.get_object("rrun").unwrap();
    let css_path = config_directory.join(Path::new("style.css"));
    let css_provider = CssProvider::new();
    if css_provider.load_from_path(css_path.to_str().unwrap()).is_err() {
        debug!("unable to load CSS!");
    };
    let screen = window.get_screen().unwrap();
    StyleContext::add_provider_for_screen(&screen, &css_provider, 1);
    let completion_list: TreeView = builder.get_object("completion_view").unwrap();
    let entry: SearchEntry = builder.get_object("search_entry").unwrap();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.set_border_width(0);
    window.set_decorated(false);
    window.show_all();
    let column_types = [glib::Type::String];
    let completion_store = ListStore::new(&column_types);

    completion_list.set_model(Some(&completion_store));
    completion_list.set_headers_visible(false);

    append_text_column(&completion_list);

    let last_pressed_key: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let current_completion_index: Rc<Cell<i32>> = Rc::new(Cell::new(0));

    env_logger::init().unwrap_or_else(|x| panic!("Error initializing logger: {}", x));

    let current_completions: Rc<RefCell<Vec<Completion>>> = Rc::new(RefCell::new(vec![]));
    let selected_completion: Rc<RefCell<Option<Completion>>> = Rc::new(RefCell::new(None));
    let current_and_selected_completions = (current_completions.clone(), selected_completion.clone());
    completion_list.get_selection().connect_changed(move |tree_selection| {
        if let Some((completion_model, iter)) = tree_selection.get_selected() {
            if let Some(path) = completion_model.get_path(&iter) {
                let selected_number = usize::from_str(path.to_string().trim()).unwrap();
                let (ref current_completions, ref selected_completion) = current_and_selected_completions;
                *selected_completion.borrow_mut() = Some(current_completions.borrow()[selected_number].clone());
            }
        }
    });

    window.connect_key_release_event(move |_, key| {
        let keyval = key.get_keyval();
        let keystate = key.get_state();
        // let keystate = (*key).state;
        debug!("key pressed: {}", keyval);
        match keyval {
            key::Escape => gtk::main_quit(),
            key::Return => {
                debug!("keystate: {:?}", keystate);
                debug!("Controlmask == {:?}", gdk::CONTROL_MASK);
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

                if keystate.intersects(gdk::CONTROL_MASK) {
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
                if last_pressed_key.get() == key::Tab {
                    current_completion_index.set(current_completion_index.get() + 1);
                    if let Some(ref c) = current_completions.borrow().get(current_completion_index.get() as usize) {
                        entry.set_text(&c.id);
                    }
                } else {
                    // if last pressed key wasn't tab, we fill the entry with the most likely completion
                    if let Some(ref c) = current_completions.borrow().get(0) {
                        entry.set_text(&c.id);
                        current_completion_index.set(0);
                    }
                }
            }
            _ => {
                let is_text_modifying = keyval_to_unicode(key.get_keyval()).is_some() || keyval == key::BackSpace ||
                                        keyval == key::Delete;
                if is_text_modifying {
                    let query =
                        entry.get_text().unwrap_or_else(|| panic!("Unable to get string from Entry widget!")).clone();
                    let completions = engine.get_completions(query.trim()).collect_vec();
                    completion_store.clear();
                    // debug!("Found {:?} completions", completions.len());
                    for (i, cmpl) in completions.iter().enumerate().take(20) {
                        let iter = completion_store.append();
                        completion_store.set(&iter, &[0], &[&format!("{}. {}", i + 1, cmpl.text).trim()]);
                    }
                    *current_completions.borrow_mut() = completions;
                    let tree_selection = completion_list.get_selection();
                    let select_path = TreePath::new_first();
                    tree_selection.select_path(&select_path);
                }
            }
        }
        last_pressed_key.set(key.get_keyval());
        Inhibit(false)
    });

    gtk::main();
}
