//! wrote my own logger because I wanted to make it pretty

use chrono::prelude::*;
use lliw::{Fg, Reset};
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

/// Named stake for the obvious vampire + log joke
struct Stake;
impl log::Log for Stake {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let time: DateTime<Local> = Local::now();

            let time = format!(
                "{} / {}:{}:{}.{}",
                time.date(),
                time.hour(),
                time.minute(),
                time.second(),
                time.timestamp_subsec_millis()
            );

            let color = match record.level() {
                log::Level::Error => Fg::Red,
                log::Level::Warn => Fg::LightYellow,
                log::Level::Info => Fg::LightWhite,
                log::Level::Debug => Fg::Blue,
                log::Level::Trace => Fg::Yellow,
            };
            println!(
                "[{}{}{}][{}{}{}] {}",
                color,
                record.level(),
                Fg::Reset,
                Fg::Green,
                time,
                Reset,
                record.args()
            );
        }
    }
    /// Clear the log buffer
    fn flush(&self) {}
}

/// initialize the logger
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&Stake).map(|()| log::set_max_level(LevelFilter::Trace))
}
