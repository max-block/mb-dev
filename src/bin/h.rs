use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: h COMMAND [SERVER]");
        process::exit(1);
    }

    let command = args.get(1).unwrap();

    match command.as_str() {
        "l" => {
            dev_cli::shell_exec(
                "hcloud server list -o columns=name,ipv4,datacenter,status,type,volumes",
            );
        }
        "r" => {
            if args.len() != 3 {
                println!("Usage: h r SERVER");
                process::exit(1);
            }
            let server = args.get(2).unwrap();
            dev_cli::shell_exec(&format!(
                "hcloud server rebuild {} --image=ubuntu-20.04",
                server
            ));
        }
        _ => {
            println!("unknown command: {}", command);
            process::exit(1);
        }
    }
}
