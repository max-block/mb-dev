use std::env;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: p COMMAND [SERVER]");
        process::exit(1);
    }

    let command = args.get(1).unwrap();

    match command.as_str() {
        "o" => {
            dev_cli::shell_exec("pip list -o");
        }
        "l" => {
            dev_cli::shell_exec("pip list");
        }
        "u" => {
            dev_cli::shell_exec("pip install -U pip setuptools wheel");
        }
        "i" => {
            if env::var_os("VIRTUAL_ENV").is_none() {
                println!("venv is not activated");
                process::exit(1);
            }
            if args.len() > 2 {
                for package in &args[2..] {
                    dev_cli::shell_exec(&format!("pip install {}", package));
                }
            } else if Path::new("setup.py").exists() {
                dev_cli::shell_exec("pip install -Ue .[dev]");
            } else {
                dev_cli::shell_exec("pip install -Ur requirements.txt");
            }
        }
        "v" => {
            if env::var_os("VIRTUAL_ENV").is_some() {
                println!("venv is activated already");
                process::exit(1);
            }
            if Path::new(".venv").exists() {
                println!(".venv exists already");
                process::exit(1);
            }
            dev_cli::shell_exec("python -m venv .venv");
        }
        "d" => {
            if env::var_os("VIRTUAL_ENV").is_none() {
                println!("venv is not activated");
                process::exit(1);
            }
            dev_cli::shell_exec("pip list --format freeze -e | xargs pip uninstall -y");
            dev_cli::shell_exec("pip freeze | xargs pip uninstall -y");
        }
        _ => {
            println!("unknown command: {}", command);
            process::exit(1);
        }
    }
}
