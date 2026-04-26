//! Structured logging with configurable severity levels.
//!
//! Messages are written to `stderr` via [`log`]. The global level
//! is controlled by [`set_level`] / [`current_level`].

use std::sync::atomic::{AtomicU8, Ordering};

static LOG_LEVEL: AtomicU8 = AtomicU8::new(2);

/// Log severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Level {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Off = 5,
}

impl Level {
    /// Returns the textual representation of the level.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Trace => "TRACE",
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
            Self::Off => "OFF",
        }
    }
}

/// Sets the global log level.
pub fn set_level(level: Level) {
    LOG_LEVEL.store(level as u8, Ordering::Relaxed);
}

/// Returns the current log level.
pub fn current_level() -> Level {
    match LOG_LEVEL.load(Ordering::Relaxed) {
        0 => Level::Trace,
        1 => Level::Debug,
        2 => Level::Info,
        3 => Level::Warn,
        4 => Level::Error,
        _ => Level::Off,
    }
}

/// Writes a message to `stderr` if `level` ≥ current level.
pub fn log(level: Level, module: &str, message: &str) {
    if level >= current_level() {
        eprintln!("[{}] {}: {}", level.as_str(), module, message);
    }
}

/// Logs at `Trace` level.
pub fn trace(module: &str, message: &str) {
    log(Level::Trace, module, message);
}
/// Logs at `Debug` level.
pub fn debug(module: &str, message: &str) {
    log(Level::Debug, module, message);
}
/// Logs at `Info` level.
pub fn info(module: &str, message: &str) {
    log(Level::Info, module, message);
}
/// Logs at `Warn` level.
pub fn warn(module: &str, message: &str) {
    log(Level::Warn, module, message);
}
/// Logs at `Error` level.
pub fn error(module: &str, message: &str) {
    log(Level::Error, module, message);
}
