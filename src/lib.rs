use std::process;
use std::process::{Command, Stdio};

pub const VERSION: &str = "0.1";

pub fn shell_exec(shell_command: &str) {
    println!("{}", shell_command);
    let output = Command::new("sh")
        .args(&["-c", shell_command])
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output()
        .expect("shell_exec failed");
    let res = std::str::from_utf8(&output.stdout).expect("shell_exec->from_utf8 failed");
    println!("{}", res);
}
