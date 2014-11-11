use std::io::Command;
use std::io::process;


pub fn execute(cmd: String, forget: bool) -> Option<String> {
    debug!("executing: {}", cmd);
    let mut process = match Command::new("bash").arg("-c").arg(cmd).spawn() {
      Ok(p) => p,
      Err(e) => panic!("failed to execute process: {}", e),
    };
    if forget {
        process.forget();
        return None;
    }

    let output = process.stdout.as_mut().unwrap().read_to_end().unwrap();
    let out_str = String::from_utf8_lossy(output.as_slice()).into_string();
    let result = process.wait();
    match result {
        Ok(process::ExitStatus(0)) => Some(out_str),
        _ => None
    }

}