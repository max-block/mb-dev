use clap::{arg, crate_version, Command};

use mb_dev::{exit, shell, user_input};

fn main() {
    let matches = Command::new("hcloud helper")
        .version(crate_version!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("list").alias("l").about("List servers"))
        .subcommand(Command::new("rebuild").alias("r").about("Rebuild a server").arg(arg!(<server>)))
        .subcommand(Command::new("delete").alias("d").about("Delete a server").arg(arg!(<server>)))
        .get_matches();

    match matches.subcommand() {
        Some(("list", _)) => shell("hcloud server list -o columns=name,ipv4,datacenter,status,type,volumes"),
        Some(("rebuild", m)) => {
            let server = m.value_of("server").unwrap();

            // Rebuild only 'test' server without a confirmation check
            if server != "test" {
                let confirm = user_input("Sure? Type the server name again: ");
                if server != confirm {
                    exit(&format!("Confirm failed! {} != {}", server, confirm))
                }
            }
            shell(&format!("hcloud server rebuild '{}' --image=ubuntu-22.04", server));
            shell(&format!("dh {}", server));
        }
        Some(("delete", m)) => {
            let server = m.value_of("server").unwrap();
            if server == "test" {
                exit("Can't delete 'test' server. Do it via hcloud directly.")
            }
            let confirm = user_input("Sure? Type the server name again: ");
            if server != confirm {
                exit(&format!("Confirm failed! {} != {}", server, confirm))
            }
            shell(&format!("hcloud server delete '{}'", server))
        }
        _ => println!("unknown subcommand"),
    }
}
