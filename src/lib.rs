use std::{
    io::{self, Write},
    process::{self, Command, Stdio},
};

pub fn shell_exec(shell_command: &str) -> String {
    let output = Command::new("sh").args(&["-c", shell_command]).output().expect("shell_exec failed");
    std::str::from_utf8(&output.stdout).expect("shell_exec->from_utf8 failed").to_string()
}

pub fn shell_print(shell_command: &str) {
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
