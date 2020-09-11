use std::process;

pub const VERSION: &str = "0.1";

pub fn shell_exec(shell_command: &str) {
    println!("{}", shell_command);
    let output = process::Command::new("sh")
        .args(&["-c", shell_command])
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stdin(std::process::Stdio::piped())
        .output()
        .expect("shell_exec failed");
    let res = std::str::from_utf8(&output.stdout).expect("shell_exec->from_utf8 failed");
    println!("{}", res);
}
