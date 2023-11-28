use std::process::ExitCode;

mod cli;
mod package_navi;

fn main() -> ExitCode {
    match std::env::var("RUST_LOG") {
        Ok(_) => (),
        Err(_) => {
            std::env::set_var("RUST_LOG", "INFO");
        }
    }

    pretty_env_logger::init();
    cli::run()
}
