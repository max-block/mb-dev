use std::{
    io::{self, Write},
    process::{self, Command, Stdio},
};

pub fn shell(cmd: &str) {
    println!("{}", cmd);
    Command::new("sh")
        .args(&["-c", cmd])
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output()
        .expect("shell_exec failed");
}

pub fn shell_capture(cmd: &str) -> String {
    let output = Command::new("sh").args(&["-c", cmd]).output().expect("shell_exec failed");
    std::str::from_utf8(&output.stdout).expect("shell_exec->from_utf8 failed").to_string()
}

pub fn user_input(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn exit(msg: &str) -> ! {
    println!("{}", msg);
    process::exit(1)
}
