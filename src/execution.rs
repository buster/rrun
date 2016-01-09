use std::process::Command;
use std::process::Output;

pub fn execute(cmd: String, in_background: bool) -> Result<String, String> {
    if in_background {
        debug!("executing in background: {}", cmd);
        match Command::new("bash")
                  .arg("-c")
                  .arg(&cmd)
                  .spawn() {
            Ok(_) => return Ok("".to_owned()),
            Err(x) => return Err(format!("failed to run command {} in the background: {}", cmd, x)),
        }
    } else {
        debug!("executing and getting stdout: {}", cmd);
        let Output {status, stdout, .. } = match Command::new("bash")
                                                     .arg("-c")
                                                     .arg(&cmd)
                                                     .output() {
            Ok(output) => output,
            Err(x) => return Err(format!("unable to get output: {}!", x)),
        };
        let out = String::from_utf8_lossy(&stdout).into_owned();
        debug!("out: {}", out);
        if status.success() {
            return Ok(out);
        }
    }

    Err(format!("Something went wrong, executing {}", cmd))
}
