use std::env;

use config::{AppConfig, Frontend};

fn main() {
    let config = AppConfig::from_args(env::args().skip(1));

    logging::init();

    let exit_code = match config.frontend {
        Frontend::Tui => tui::run(),
    };

    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}
