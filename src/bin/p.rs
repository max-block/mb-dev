use clap::{App, Arg, SubCommand};
use mb_dev::VERSION;
use std::env;
use std::path::Path;
use std::process;

fn main() {
    let list_sub = SubCommand::with_name("list")
        .about("List installed packages")
        .alias("l");
    let list_outdated_sub = SubCommand::with_name("list-outdated")
        .about("List outdated packages")
        .alias("o");
    let update_sub = SubCommand::with_name("update")
        .about("Update pip, setuptools, wheel")
        .alias("u");
    let venv_sub = SubCommand::with_name("venv")
        .about("Create venv")
        .alias("v");
    let install_sub = SubCommand::with_name("install")
        .about("Create venv")
        .alias("i")
        .arg(
            Arg::with_name("packages")
                .required(false)
                .takes_value(true)
                .multiple(true),
        );

    let delete_sub = SubCommand::with_name("delete")
        .about("Delete installed packages for venv")
        .alias("d");

    let matches = App::new("python shortcuts")
        .version(VERSION)
        .subcommand(list_sub)
        .subcommand(list_outdated_sub)
        .subcommand(update_sub)
        .subcommand(venv_sub)
        .subcommand(install_sub)
        .subcommand(delete_sub)
        .get_matches();

    match matches.subcommand() {
        ("list", Some(_)) => {
            mb_dev::shell_exec("pip list");
        }
        ("list-outdated", Some(_)) => {
            mb_dev::shell_exec("pip list -o");
        }
        ("update", Some(_)) => {
            mb_dev::shell_exec("pip install -U pip setuptools wheel");
        }
        ("venv", Some(_)) => {
            if env::var_os("VIRTUAL_ENV").is_some() {
                println!("venv is activated already");
                process::exit(1);
            }
            if Path::new(".venv").exists() {
                println!(".venv exists already");
                process::exit(1);
            }
            mb_dev::shell_exec("python -m venv .venv");
        }
        ("install", Some(sub_matches)) => {
            if env::var_os("VIRTUAL_ENV").is_none() {
                println!("venv is not activated");
                process::exit(1);
            }
            if sub_matches.values_of("packages").is_some() {
                for package in sub_matches.values_of("packages").unwrap() {
                    mb_dev::shell_exec(&format!("pip install {}", package));
                }
            } else if Path::new("setup.py").exists() {
                mb_dev::shell_exec("pip install -Ue .[dev]");
            } else {
                mb_dev::shell_exec("pip install -Ur requirements.txt");
            }
        }
        ("delete", Some(_)) => {
            if env::var_os("VIRTUAL_ENV").is_none() {
                println!("venv is not activated");
                process::exit(1);
            }
            mb_dev::shell_exec("pip list --format freeze -e | xargs pip uninstall -y");
            mb_dev::shell_exec("pip freeze | xargs pip uninstall -y");
        }
        _ => {}
    }
}
