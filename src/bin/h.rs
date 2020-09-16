use clap::{App, Arg, SubCommand};

use dev_cli::VERSION;

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
                dev_cli::shell_exec(&format!(
                    "hcloud server rebuild {} --image=ubuntu-20.04",
                    server
                ));
            }
        }
        ("delete", Some(sub_matches)) => {
            for server in sub_matches.values_of("servers").unwrap() {
                dev_cli::shell_exec(&format!("hcloud server delete {}", server));
            }
        }
        _ => {}
    }
}
