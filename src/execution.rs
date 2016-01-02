use std::process::Command;
use std::process::Output;

pub fn execute(cmd: String, forget_stdout: bool) -> Option<String> {
    debug!("executing: {}", cmd);
    if !cmd.starts_with("compgen") && !cmd.starts_with("history ") {
        let mut hist_cmd = "(history -s ".to_owned();
        hist_cmd.push_str(&cmd);
        hist_cmd.push_str("; history -a)");
        Command::new("bash")
            .arg("-i")
            .arg("-c")
            .arg(&hist_cmd)
            .output()
            .unwrap_or_else(|_| panic!("unable to append to history!"));
    }

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
