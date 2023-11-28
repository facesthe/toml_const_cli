use std::process::ExitCode;

mod cli;
mod package_navi;

fn main() -> ExitCode {
    cli::run()
}
