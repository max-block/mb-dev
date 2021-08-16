use std::{env, path::Path};

use clap::{App, AppSettings, Arg, crate_version};

use mb_dev::{exit, shell, shell_capture};

fn process_install(packages: Vec<&str>) {
    if env::var_os("VIRTUAL_ENV").is_none() {
        exit("venv is not activated")
    }

    if !packages.is_empty() {
        shell(&format!("pip install {}", packages.join(" ")))
    } else if Path::new("setup.py").exists() {
        shell("pip install -Ue .[dev]")
    } else {
        shell("pip install -Ur requirements.txt")
    }
}

fn process_venv() {
    if env::var_os("VIRTUAL_ENV").is_some() {
        exit("venv is activated already")
    }
    if Path::new(".venv").exists() {
        exit(".venv exists already")
    }
    shell("python3 -m venv .venv")
}

fn process_uninstall() {
    if env::var_os("VIRTUAL_ENV").is_none() {
        exit("venv is not activated")
    }
    shell("pip list --format freeze -e | xargs pip uninstall -y");
    shell("pip freeze | xargs pip uninstall -y");
}

fn process_kill() {
    for p in find_python_processes() {
        shell(&format!("kill -9 {}", p))
    }
}

fn find_python_processes() -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    let res = shell_capture(r#"echo "$(ps -o pid,command -ax)""#);
    for line in res.split('\n') {
        if line.contains("python")
            && (line.contains("uvicorn")
                || line.contains("multiprocessing.resource_tracker")
                || line.contains("multiprocessing.spawn"))
        {
            if let Some(number) = line.split_ascii_whitespace().next() {
                result.push(number.parse::<u32>().unwrap());
            }
        }
    }
    result
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
                .arg(Arg::new("packages").required(false).multiple_values(true)),
        )
        .subcommand(App::new("venv").alias("v").about("create .venv"))
        .subcommand(App::new("uninstall").alias("d").about("uninstall all packages(+editable) from venv"))
        .subcommand(App::new("kill").alias("k").about("kill python processes"))
        .get_matches();

    match matches.subcommand() {
        Some(("pip-list-outdated", _)) => shell("pip list -o"),
        Some(("pip-list", _)) => shell("pip list"),
        Some(("pip-update", _)) => shell("pip install -U pip setuptools"),
        Some(("install", m)) => process_install(m.values_of("packages").unwrap_or_default().collect::<Vec<&str>>()),
        Some(("venv", _)) => process_venv(),
        Some(("uninstall", _)) => process_uninstall(),
        Some(("kill", _)) => process_kill(),
        _ => println!("unsupported command"),
    }
}
