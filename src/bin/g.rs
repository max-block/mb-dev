use clap::{App, Arg, SubCommand};
use dev_cli::VERSION;

fn main() {
    let diff_sub = SubCommand::with_name("diff").about("git diff").alias("d");
    let tag_sub = SubCommand::with_name("tag").about("git tag").alias("t");
    let status_sub = SubCommand::with_name("status")
        .about("git status")
        .alias("s");
    let clone_sub = SubCommand::with_name("clone")
        .about("git clone")
        .alias("c")
        .arg(Arg::with_name("url").required(true).takes_value(true));
    let push_sub = SubCommand::with_name("push")
        .about("git add . && git commit -m update && git push")
        .alias("p");
    let add_tag_sub = SubCommand::with_name("add-tag")
        .about("add tag")
        .alias("at")
        .arg(Arg::with_name("version").required(true).takes_value(true));
    let delete_tag_sub = SubCommand::with_name("delete-tag")
        .about("delete tag")
        .alias("dt")
        .arg(Arg::with_name("version").required(true).takes_value(true));

    let matches = App::new("hcloud shortcuts")
        .version(VERSION)
        .subcommand(diff_sub)
        .subcommand(tag_sub)
        .subcommand(status_sub)
        .subcommand(clone_sub)
        .subcommand(push_sub)
        .subcommand(add_tag_sub)
        .subcommand(delete_tag_sub)
        .get_matches();

    match matches.subcommand() {
        ("diff", Some(_)) => dev_cli::shell_exec("git diff"),
        ("tag", Some(_)) => dev_cli::shell_exec("git tag --sort=-creatordate"),
        ("status", Some(_)) => dev_cli::shell_exec("git status"),
        ("push", Some(_)) => dev_cli::shell_exec("git add . && git commit -m update && git push"),
        ("clone", Some(sub_matches)) => {
            dev_cli::shell_exec(&format!(
                "git clone {}",
                sub_matches.value_of("url").unwrap()
            ));
        }
        ("add-tag", Some(sub_matches)) => {
            let version = sub_matches.value_of("version").unwrap();
            dev_cli::shell_exec(&format!(
                "git tag -a '{version}' -m 'version {version}' && git push origin {version}",
                version = version
            ));
        }
        ("delete-tag", Some(sub_matches)) => {
            let version = sub_matches.value_of("version").unwrap();
            dev_cli::shell_exec(&format!(
                "git tag -d '{version}' && git push origin :refs/tags/{version}",
                version = version
            ));
        }
        _ => {}
    }
}
