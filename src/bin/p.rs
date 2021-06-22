use std::{env, path::Path};

use clap::{crate_version, App, AppSettings, Arg};
use mb_dev::{exit, shell_exec};

fn process_install(packages: Vec<&str>) {
    if env::var_os("VIRTUAL_ENV").is_none() {
        exit("venv is not activated")
    }

    if !packages.is_empty() {
        shell_exec(&format!("pip install {}", packages.join(" ")))
    } else if Path::new("setup.py").exists() {
        shell_exec("pip install -Ue .[dev]")
    } else {
        shell_exec("pip install -Ur requirements.txt")
    }
}

fn process_venv() {
    if env::var_os("VIRTUAL_ENV").is_some() {
        exit("venv is activated already")
    }
    if Path::new(".venv").exists() {
        exit(".venv exists already")
    }
    shell_exec("python3 -m venv .venv")
}

fn process_uninstall() {
    if env::var_os("VIRTUAL_ENV").is_none() {
        exit("venv is not activated")
    }
    shell_exec("pip list --format freeze -e | xargs pip uninstall -y");
    shell_exec("pip freeze | xargs pip uninstall -y");
}

fn process_kill_uvicorn() {
    println!("is not impletemted yet!")
}

fn main() {
    let matches = App::new("python helper")
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::DisableHelpSubcommand)
        .subcommand(App::new("pip-list-outdated").alias("o").about("pip list -o"))
        .subcommand(App::new("pip-list").alias("l").about("pip list"))
        .subcommand(App::new("pip-update").alias("u").about("pip install -U pip setuptools"))
        .subcommand(
            App::new("install")
                .alias("i")
                .about("install packages or project (setup.py or requirements.txt)")
                .arg(Arg::new("packages").required(false).multiple(true)),
        )
        .subcommand(App::new("venv").alias("v").about("create .venv"))
        .subcommand(App::new("uninstall").alias("d").about("uninstall all packages(+editable) from venv"))
        .subcommand(App::new("kill-uvicorn").alias("k").about("kill a dev uvicorn server"))
        .get_matches();

    match matches.subcommand() {
        Some(("pip-list-outdated", _)) => shell_exec("pip list -o"),
        Some(("pip-list", _)) => shell_exec("pip list"),
        Some(("pip-update", _)) => shell_exec("pip install -U pip setuptools"),
        Some(("install", m)) => process_install(m.values_of("packages").unwrap_or_default().collect::<Vec<&str>>()),
        Some(("venv", _)) => process_venv(),
        Some(("uninstall", _)) => process_uninstall(),
        Some(("", _)) => process_kill_uvicorn(),
        _ => println!("unsupported command"),
    }
}
