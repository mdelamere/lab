use simplelog::*;
use std::fs::File;

pub fn init_logging(log_file: &str) {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Info, Config::default(), File::create(log_file).unwrap()),
    ]).unwrap();
}
