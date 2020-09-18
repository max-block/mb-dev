use clap::{App, Arg, SubCommand};
use std::io;

use dev_cli::VERSION;
use std::io::Write;

fn main() {
    let list_sub = SubCommand::with_name("list")
        .about("List servers")
        .alias("l");

    let rebuild_sub = SubCommand::with_name("rebuild")
        .about("Rebuild servers")
        .alias("r")
        .arg(
            Arg::with_name("servers")
                .required(true)
                .takes_value(true)
                .multiple(true),
        );

    let delete_sub = SubCommand::with_name("delete")
        .about("Delete servers")
        .alias("d")
        .arg(
            Arg::with_name("servers")
                .required(true)
                .takes_value(true)
                .multiple(true),
        );

    let matches = App::new("hcloud shortcuts")
        .version(VERSION)
        .subcommand(list_sub)
        .subcommand(rebuild_sub)
        .subcommand(delete_sub)
        .get_matches();

    match matches.subcommand() {
        ("list", Some(_)) => dev_cli::shell_exec(
            "hcloud server list -o columns=name,ipv4,datacenter,status,type,volumes",
        ),
        ("rebuild", Some(sub_matches)) => {
            for server in sub_matches.values_of("servers").unwrap() {
                if server != "test" {
                    print!(
                    "You are going to rebuild the server \"{}\". If you are sure, type its name: ",
                    server
                );
                    let _ = io::stdout().flush();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    if server == input.trim() {
                        dev_cli::shell_exec(&format!(
                            "hcloud server rebuild {} --image=ubuntu-20.04",
                            server
                        ));
                    } else {
                        println!("You've typed a wrong name!");
                    }
                } else {
                    dev_cli::shell_exec(&format!(
                        "hcloud server rebuild {} --image=ubuntu-20.04",
                        server
                    ));
                }
            }
        }
        ("delete", Some(sub_matches)) => {
            for server in sub_matches.values_of("servers").unwrap() {
                print!(
                    "You are going to delete the server \"{}\". If you are sure, type its name: ",
                    server
                );
                let _ = io::stdout().flush();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if server == input.trim() {
                    dev_cli::shell_exec(&format!("hcloud server delete {}", server));
                } else {
                    println!("You've typed a wrong name!");
                }
            }
        }
        _ => {}
    }
}
