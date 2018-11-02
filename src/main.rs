#![feature(crate_visibility_modifier)]

use {
    crate::config::Config,
    getopts::{HasArg, Occur, Options},
    std::{error::Error, ffi::OsStr, path::Path, path::PathBuf, process::Command},
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

fn run() -> Result<(), Box<Error>> {
    let conf = load_conf()?;

    let mut args = std::env::args();
    let program = args.next().unwrap();
    let mut opts = Options::new();
    opts.opt(
        "r",
        "run",
        "Run a binary after building",
        "Hint? Wut?",
        HasArg::Maybe,
        Occur::Optional,
    );
    opts.optflag("h", "help", "print this help menu");
    let mut my_args = Vec::new();
    while let Some(s) = args.next() {
        if s == "--" {
            break;
        } else {
            my_args.push(s);
        }
    }

    let matches = match opts.parse(my_args) {
        Ok(m) => m,
        Err(e) => return Err(e.into()),
    };

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return Ok(());
    }

    let invocation_path = std::env::current_dir()?;

    let target_name = match matches.free.get(0) {
        Some(name) => name,
        None => {
            if conf.default_target.is_empty() {
                return Err("No target specified and no default-target in `cmakr.toml`.".into());
            } else {
                &conf.default_target
            }
        }
    };

    build_target(&conf, target_name)?;
    if matches.opt_present("run") {
        let bin_name = match matches.opt_str("run") {
            Some(name) => name,
            None => {
                if conf.default_bin.is_empty() {
                    return Err("No binary specified and no default-bin in `cmakr.toml`".into());
                } else {
                    conf.default_bin
                }
            }
        };
        exec(&bin_name, &invocation_path, args)?;
    }
    Ok(())
}

#[cfg(not(unix))]
fn exec<I, S>(bin_name: &str, wd_path: &Path, args: I) -> Result<(), Box<Error>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new(std::env::current_dir()?.join(bin_name))
        .current_dir(wd_path)
        .args(args)
        .status()?;
    Ok(())
}

#[cfg(unix)]
fn exec<I, S>(bin_name: &str, wd_path: &Path, args: I) -> Result<(), Box<Error>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    use std::os::unix::process::CommandExt;
    let bin_path = std::env::current_dir()?.join(bin_name);
    if !bin_path.exists() {
        return Err(format!("{:?} doesn't exist", bin_path).into());
    }
    Command::new(bin_path)
        .current_dir(wd_path)
        .args(args)
        .exec();
    // If we are at this point exec failed to replace our process, abort.
    std::process::abort();
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
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
