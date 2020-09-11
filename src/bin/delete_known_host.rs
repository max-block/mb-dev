use std::env;
use std::process;

use dns_lookup::lookup_host;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: delete_known_host HOST");
        process::exit(1);
    }

    let host = args.get(1).expect("");
    dev_cli::shell_exec(format!("ssh-keygen -R {}", host.as_str()).as_str());

    if let Ok(ips) = lookup_host(host) {
        for h in ips.iter() {
            dev_cli::shell_exec(format!("ssh-keygen -R {}", h.to_string()).as_str());
        }
    }
}
