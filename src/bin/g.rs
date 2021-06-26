use clap::{crate_version, App, AppSettings, Arg};
use mb_dev::shell_print;

fn main() {
    let matches = App::new("git helper")
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::DisableHelpSubcommand)
        .subcommand(App::new("diff").alias("d").about("git diff"))
        .subcommand(App::new("log").alias("l").about("git log"))
        .subcommand(App::new("tag").alias("t").about("git tag"))
        .subcommand(App::new("status").alias("s").about("git status"))
        .subcommand(App::new("clone").alias("c").about("git clone").arg("<repo>"))
        .subcommand(App::new("push").alias("p").about("git add & commit & push").arg(Arg::new("MESSAGE").default_value("update")))
        .subcommand(App::new("add-tag").alias("at").about("add tag and push").arg("<VERSION>"))
        .subcommand(App::new("delete-tag").alias("dt").about("delete tag and push").arg("<VERSION>"))
        .get_matches();

    match matches.subcommand() {
        Some(("diff", _)) => shell_print("git diff"),
        Some(("log", _)) => shell_print("git log"),
        Some(("tag", _)) => shell_print("git tag --sort=-creatordate"),
        Some(("status", _)) => shell_print("git status"),
        Some(("clone", m)) => shell_print(&format!("git clone {}", m.value_of("repo").unwrap())),
        Some(("push", m)) => shell_print(&format!("git add . && git commit -m '{}' && git push", m.value_of("MESSAGE").unwrap())),
        Some(("add-tag", m)) => shell_print(&format!(
            "git tag -a '{version}' -m '{version}' && git push origin {version}",
            version = m.value_of("VERSION").unwrap()
        )),
        Some(("delete-tag", m)) => shell_print(&format!(
            "git tag -d '{version}' && git push origin :refs/tags/{version}",
            version = m.value_of("VERSION").unwrap()
        )),
        _ => println!("unknown subcommand"),
    }
}
