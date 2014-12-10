use std::io::Command;
use std::io::process;
use std::os;
use std::io::{File, Append, Write};

fn append_to_history(cmd: &str) {
    let h_file = os::getenv("HISTFILE").unwrap_or(".bash_history".to_string());
    let h_dir = os::homedir().unwrap_or_else(|| { panic!("unable to get homedir!")});
    let h_file_p = h_dir.join(h_file);
    println!("history file: {}", h_file_p.display());
    let mut file = match File::open_mode(&h_file_p, Append, Write) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };
    file.write_line(cmd);
    file.fsync();
}

pub fn execute(cmd: String, forget: bool) -> Option<String> {
    debug!("executing: {}", cmd);
    let mut process = match Command::new("bash").arg("-c").arg(cmd.clone()).spawn() {
      Ok(p) => p,
      Err(e) => panic!("failed to execute process: {}", e),
    };
    append_to_history(cmd.as_slice());
    if forget {
        process.forget();
        return None;
    }

    let output = process.stdout.as_mut().unwrap().read_to_end().unwrap();
    let out_str = String::from_utf8_lossy(output.as_slice()).into_string();
    let result = process.wait();
    debug!("result: {}", result);
    match result {
        Ok(process::ExitStatus(0)) => Some(out_str),
        _ => None
    }

}
