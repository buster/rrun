use std::old_io::Command;
use std::old_io::process;
use std::env;
use std::old_io::{File, Append, Write, IoResult};

fn append_to_history(cmd: &str) -> IoResult<()> {
    let h_file = env::var("HISTFILE").unwrap_or(".bash_history".to_string());
    let h_dir = env::home_dir().unwrap_or_else(|| { panic!("unable to get homedir!") } );
    let h_file_p = h_dir.join(h_file);
    debug!("opening history file: {}", h_file_p.display());
    let mut file = try!(File::open_mode(&h_file_p, Append, Write));
    try!(file.write_line(cmd));
    try!(file.fsync());
    return Ok(());
}

pub fn execute(cmd: String, forget: bool) -> Option<String> {
    debug!("executing: {}", cmd);
    let mut process = match Command::new("bash").arg("-c").arg(cmd.clone()).spawn() {
      Ok(p) => p,
      Err(e) => panic!("failed to execute process: {}", e),
    };
    let _ = match append_to_history(&cmd) {
        Err(e) => error!("Unable to append to history: {}", e),
        Ok(()) => debug!("wrote to history")
    };

    if forget {
        process.forget();
        return None;
    }

    let output = process.stdout.as_mut().unwrap().read_to_end().unwrap();
    let out_str = String::from_utf8_lossy(&output).to_string();
    let result = process.wait();
    debug!("result: {:?}", result);
    match result {
        Ok(process::ExitStatus(0)) => Some(out_str),
        _ => None
    }

}
