#![feature(globs)]
#![crate_type = "bin"]

extern crate rgtk;
extern crate libc;

use rgtk::*;
use rgtk::gtk::signals;
use std::io::Command;


fn complete(cmd: String, last_cmd: Option<String>) -> String {
    let output = execute(format!("compgen -A command {}", cmd));
    {
        let mut possible_commands = output.lines();
        let mut last_iter_cmd = "".into_string();
        // return first match if no previous one was matched
        println!("cmd: {}, last_cmd: {}, possible_commands:  {}", cmd, last_cmd, output);
        if last_cmd.is_none() { 
            let cmd_arr: Vec<&str> = possible_commands.collect();
            return cmd_arr[0].into_string()
        };
        let last_cmd = last_cmd.unwrap();
        for cmd in possible_commands {
            // return current item if last item was used previously
            if last_iter_cmd == last_cmd { 
                return cmd.into_string() 
            };
            last_iter_cmd = cmd.into_string();
        }
       return last_iter_cmd;
    }
}


fn execute(cmd: String) -> String {
    println!("executing: {}", cmd);
    let mut process = match Command::new("bash").arg("-c").arg(cmd).spawn() {
      Ok(p) => p,
      Err(e) => panic!("failed to execute process: {}", e),
    };

    let output = process.stdout.as_mut().unwrap().read_to_end().unwrap();
    return String::from_utf8_lossy(output.as_slice()).into_string();
}

fn main() {
    gtk::init();
    println!("GTK VERSION: Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());
    let mut window = gtk::Window::new(gtk::window_type::TopLevel).unwrap();
    let mut entry = gtk::SearchEntry::new().unwrap();
    window.set_title("Yeah a beautiful window with rgtk !");
    window.set_window_position(gtk::window_position::Center);
    
    let mut last_pressed_key = 0;
    let mut last_user_cmd: String = "".into_string();
    let mut last_user_completion: Option<String> = None;

    window.connect(signals::KeyPressEvent::new(|key|{
        let keyval = unsafe { (*key).keyval };
        println!("key pressed: {}", keyval);
        match keyval {
            65307 => gtk::main_quit(),
            65293 => {
                let cmd = entry.get_text().unwrap();
                let output = execute(cmd);
                entry.set_text(output.trim().into_string());
                last_user_completion = None;
                last_user_cmd = "".into_string();
            },
            65289 => {
                let mut complete_cmd: String = "".into_string();
                if last_pressed_key == 65289 {
                    complete_cmd = last_user_cmd.clone();
                }
                else {
                    complete_cmd = entry.get_text().unwrap();
                    last_user_cmd = complete_cmd.clone();
                }
                let completion = complete(complete_cmd.clone(), last_user_completion.clone());
                last_user_completion = Some(completion.clone());
                entry.set_text(completion);
            },
            _ => ()
        }
        last_pressed_key = keyval;
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
