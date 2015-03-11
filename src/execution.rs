use std::process::Command;
use std::env;
use std::process::Output;
use std::fs;
use std::io::Result;
use std::io::Write;
use std::str::StrExt;

fn append_to_history(cmd: &str) -> Result<()> {
    let h_file = env::var("HISTFILE").unwrap_or(".bash_history".to_string());
    let mut h_path = env::home_dir().unwrap_or_else(|| { panic!("unable to get homedir!") } );
    h_path.push(&h_file);
    debug!("opening history file: {:?}", h_path);
    let mut file = try!(fs::OpenOptions::new().write(true).append(true).open(&h_path));

    try!(file.write(cmd.as_bytes()));
    try!(file.write("\n".as_bytes()));
    try!(file.sync_data());
    return Ok(());
}

pub fn execute(cmd: String, forget_stdout: bool) -> Option<String> {
    debug!("executing: {}", cmd);
    if !cmd.starts_with("compgen") {
        let _ = match append_to_history(&cmd) {
            Err(e) => error!("Unable to append to history: {}", e),
            Ok(()) => debug!("wrote to history")
        };
    }

    if forget_stdout {
        Command::new("bash").arg("-c").arg(&cmd).spawn().unwrap_or_else(|_| {panic!("unable to spawn!")});
        return None;
    }
    else {
        let Output {status, stdout, .. } = Command::new("bash").arg("-c").arg(&cmd).output().unwrap();
        println!("out: {:?}", stdout);
        if status.success() {
            return Some(String::from_utf8_lossy(&stdout).into_owned())
        }
    }

    None
}
