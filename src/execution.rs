use std::process::Command;
use std::process::Output;

pub fn execute(cmd: String, forget_stdout: bool) -> Option<String> {
    debug!("executing: {}", cmd);
    if forget_stdout {
        Command::new("bash")
            .arg("-c")
            .arg(&cmd)
            .spawn()
            .unwrap_or_else(|_| panic!("unable to spawn!"));
        return None;
    } else {
        let Output {status, stdout, .. } = Command::new("bash")
                                               .arg("-c")
                                               .arg(&cmd)
                                               .output()
                                               .unwrap();
        let out = String::from_utf8_lossy(&stdout).into_owned();
        debug!("out: {}", out);
        if status.success() {
            return Some(out);
        }
    }

    None
}
