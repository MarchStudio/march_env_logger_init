#[macro_use]
extern crate log;
use env_logger::Env;
use log::LevelFilter;
use chrono::Local;
use std::io::Write;
use colored::Colorize;
pub fn init_logger() {
    let env = Env::default();
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            let level_string = format!("{}", record.level());
            let colored_level = match record.level() {
                log::Level::Error => level_string.red(),
                log::Level::Warn => level_string.yellow(),
                log::Level::Info => level_string.green(),
                log::Level::Debug | log::Level::Trace => level_string.cyan(),
            };

            writeln!(
                buf,
                //"{} {} [ {} ] \n{}",
                "{} {} \n{}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                colored_level,
                //record.module_path().unwrap_or("<unnamed>"),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
    
    info!("env_logger initialized.");
}