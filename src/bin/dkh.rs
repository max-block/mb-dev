use clap::{App, Arg, crate_version};
use dns_lookup::lookup_host;

use mb_dev::shell;

fn main() {
    let matches = App::new("dkh")
        .about("Delete hosts from ~/.ssh/known_hosts")
        .version(crate_version!())
        .arg(Arg::new("hosts").multiple_values(true).required(true))
        .get_matches();

    for host in matches.values_of("hosts").unwrap().collect::<Vec<_>>() {
        delete_host(host);
    }
}

fn delete_host(host: &str) {
    shell(&format!("ssh-keygen -R {}", host));

    if let Ok(ips) = lookup_host(host) {
        for h in ips.iter() {
            shell(&format!("ssh-keygen -R {}", h));
        }
    }
}
