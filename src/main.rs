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
    /// Command to execute. Arguments can be included.
    #[structopt(long, default_value = "qemu-system-x86_64 -drive format=raw,file=")]
    with_command: String,

    /// Append a suffix to the provided target. Example: .bin
    #[structopt(long)]
    with_suffix: Option<String>,

    /// Path to the target executable, provided by cargo.
    target_exec: String,
}

fn main() {
    let Args::Runner(mut args) = Args::from_args();
    args.with_command.push_str(&args.target_exec);
    if let Some(ext) = &args.with_suffix {
        args.with_command.push_str(&ext);
    }

    let mut commands = args.with_command.split_ascii_whitespace();

    let mut cmd = Command::new(commands.next().expect("Missing command"));

    cmd.args(commands);

    let _exit = cmd
        .status()
        .expect(&format!("Failed to run command: `{}`", args.with_command));
}
