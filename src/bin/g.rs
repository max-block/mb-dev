use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: g COMMAND [OPTS]");
        process::exit(1);
    }

    let command = args.get(1).unwrap();

    match command.as_str() {
        "d" => {
            dev_cli::shell_exec("git diff");
        }
        "t" => {
            dev_cli::shell_exec("git tag --sort=-creatordate");
        }
        "s" => {
            dev_cli::shell_exec("git status");
        }
        "c" => {
            if args.len() != 3 {
                println!("Usage: g c REPO");
                process::exit(1);
            }
            let repo = args.get(2).unwrap();
            dev_cli::shell_exec(&format!("git clone {}", repo));
        }
        "p" => {
            dev_cli::shell_exec("git add . && git commit -m update && git push");
        }
        "at" => {
            if args.len() != 3 {
                println!("Usage: g at VERSION");
                process::exit(1);
            }
            let version = args.get(2).unwrap();
            dev_cli::shell_exec(&format!(
                "git tag -a '{version}' -m 'version {version}' && git push origin {version}",
                version = version
            ));
        }
        "dt" => {
            if args.len() != 3 {
                println!("Usage: g dt VERSION");
                process::exit(1);
            }
            let version = args.get(2).unwrap();
            dev_cli::shell_exec(&format!(
                "git tag -d '{version}' && git push origin :refs/tags/{version}",
                version = version
            ));
        }
        _ => {
            println!("unknown command: {}", command);
            process::exit(1);
        }
    }
}
