// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(&message),
        LogLevel::Warning => warn(&message),
        LogLevel::Error => error(&message),
    }
}
pub fn info(message: &str) -> String {
    let mut text = String::from("[INFO]: ");
    text.push_str(message);
    text
}
pub fn warn(message: &str) -> String {
    let mut text = String::from("[WARNING]: ");
    text.push_str(message);
    text
}
pub fn error(message: &str) -> String {
    let mut text = String::from("[ERROR]: ");
    text.push_str(message);
    text
}
