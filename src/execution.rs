use std::process::Command;
use std::process::Output;

pub fn execute(cmd: String, in_background: bool) -> Option<String> {
    if in_background {
        debug!("executing in background: {}", cmd);
        Command::new("bash")
            .arg("-c")
            .arg(&cmd)
            .spawn()
            .unwrap_or_else(|_| panic!("unable to spawn!"));
        return None;
    } else {
        debug!("executing and getting stdout: {}", cmd);
        let Output {status, stdout, .. } = Command::new("bash")
                                               .arg("-c")
                                               .arg(&cmd)
                                               .output()
                                               .unwrap_or_else(|_| panic!("Unable to get output of {}!", cmd));
        let out = String::from_utf8_lossy(&stdout).into_owned();
        debug!("out: {}", out);
        if status.success() {
            return Some(out);
        }
    }

    None
}
