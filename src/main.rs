use cargo_metadata::MetadataCommand;
use serde::Deserialize;
use std::process::Command;
use structopt::{clap::AppSettings, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    bin_name = "cargo",
    global_settings(&[
        AppSettings::ColoredHelp,
]))]
enum Args {
    Runner(Runner),
}

#[derive(Debug, StructOpt)]
struct Runner {
    /// Path to the target executable, provided by cargo.
    target_exec: String,
}

#[derive(Debug, Deserialize)]
struct PackageMetadata {
    cargo_runner: CargoRunnerMeta,
}

#[derive(Debug, Deserialize)]
struct CargoRunnerMeta {
    command: Vec<String>,
    suffix: Option<String>,
}

fn main() {
    let Args::Runner(mut args) = Args::from_args();

    let meta = MetadataCommand::new()
        .no_deps()
        .exec()
        .expect("Failed to run cargo-metadata");
    let pkg_args: PackageMetadata = serde_json::from_value(meta.packages[0].metadata.clone())
        .expect("Missing package.metadata.cargo_runner");
    let mut pkg_args = pkg_args.cargo_runner;

    let target_exec = {
        if let Some(suffix) = &pkg_args.suffix {
            args.target_exec.push_str(&suffix);
        }
        args.target_exec
    };
    pkg_args
        .command
        .iter_mut()
        .for_each(|x| *x = x.replace("$TARGET_FILE", &target_exec));

    let _exit = Command::new(&pkg_args.command[0])
        .args(&pkg_args.command[1..])
        .status()
        .expect(&format!(
            "Failed to run command: `{:#?}`",
            &pkg_args.command
        ));
}
