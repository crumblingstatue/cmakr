#![feature(crate_visibility_modifier)]

use {
    clap::{App, Arg, SubCommand},
    crate::config::Config,
    std::{error::Error, path::PathBuf, process::Command},
};

mod config;

/// Ascend until we can find a cmakr.toml
fn find_conf_file() -> Result<PathBuf, Box<Error>> {
    loop {
        let current_dir = std::env::current_dir()?;
        if std::fs::metadata("cmakr.toml").is_ok() {
            return Ok(current_dir.join("cmakr.toml"));
        }
        match current_dir.parent() {
            Some(parent) => std::env::set_current_dir(parent)?,
            None => return Err("No cmakr.toml".into()),
        }
    }
}

fn load_conf() -> Result<Config, Box<Error>> {
    let conf_path = find_conf_file()?;
    let s = std::fs::read_to_string(conf_path)?;
    Ok(toml::from_str(&s)?)
}

fn main() -> Result<(), Box<Error>> {
    let conf = load_conf()?;
    let matches = App::new("cmakr")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("build")
                .aliases(&["b"])
                .arg(Arg::with_name("target").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("run")
                .aliases(&["r"])
                .arg(Arg::with_name("target").index(1).required(true))
                .arg(Arg::with_name("bin").index(2).required(true)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        build_target(&conf, matches.value_of("target").unwrap())?;
    } else if let Some(matches) = matches.subcommand_matches("run") {
        run_target(
            &conf,
            matches.value_of("target").unwrap(),
            matches.value_of("bin").unwrap(),
        )?;
    }

    Ok(())
}

fn build_target(conf: &Config, name: &str) -> Result<(), Box<Error>> {
    let target_info = match conf.targets.get(name) {
        Some(info) => info,
        None => return Err(format!("No target named {}.", name).into()),
    };
    let target_path = PathBuf::from(&conf.build_path).join(name);
    let already_generated = std::fs::metadata(&target_path).is_ok();
    if !already_generated {
        std::fs::create_dir_all(&target_path)?;
    }
    std::env::set_current_dir(&target_path)?;
    if !already_generated {
        // TODO: Proper root dir detection
        Command::new("cmake")
            .arg("../..")
            .args(&target_info.args)
            .status()?;
    }
    Command::new(&target_info.build_command).status()?;
    Ok(())
}

fn run_target(conf: &Config, name: &str, bin: &str) -> Result<(), Box<Error>> {
    let invocation_path = std::env::current_dir()?;
    build_target(conf, name)?;
    Command::new(std::env::current_dir()?.join(bin))
        .current_dir(invocation_path)
        .status()?;
    Ok(())
}
