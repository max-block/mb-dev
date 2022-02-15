use clap::{App, AppSettings, arg, crate_version};

use mb_dev::{exit, shell, user_input};

fn main() {
    let matches = App::new("hcloud helper")
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::DisableHelpSubcommand)
        .subcommand(App::new("list").alias("l").about("List servers"))
        .subcommand(App::new("rebuild").alias("r").about("Rebuild a server").arg(arg!(<server>)))
        .subcommand(App::new("delete").alias("d").about("Delete a server").arg(arg!(<server>)))
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
            shell(&format!("hcloud server rebuild '{}' --image=ubuntu-20.04", server));
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
