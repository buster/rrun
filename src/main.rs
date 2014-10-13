#![feature(globs)]
#![crate_type = "bin"]

extern crate rgtk;
extern crate libc;

use rgtk::*;
use rgtk::gtk::signals;
use std::io::Command;

fn main() {
    gtk::init();
    println!("Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());
    let mut window = gtk::Window::new(gtk::window_type::TopLevel).unwrap();
    let mut entry = gtk::SearchEntry::new().unwrap();
    //let search_entry = gtk::SearchEntry::new().unwrap();
    window.set_title("Yeah a beautiful window with rgtk !");
    window.set_window_position(gtk::window_position::Center);
    
    window.connect(signals::KeyPressEvent::new(|key|{
        let keyval = unsafe { (*key).keyval };
        println!("key pressed: {}", keyval);
        if keyval == 65307 {
            gtk::main_quit();
        }
        if keyval == 65293 {
            let cmd = entry.get_text().unwrap();
            println!("executing: {}", cmd);
            let mut process = match Command::new("sh").arg("-c").arg(cmd).spawn() {
              Ok(p) => p,
              Err(e) => fail!("failed to execute process: {}", e),
            };

            let output = process.stdout.as_mut().unwrap().read_to_end().unwrap();
            println!("ouput: {}", String::from_utf8_lossy(output.as_slice()));
            entry.set_text("".to_string());
        }
        false
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
