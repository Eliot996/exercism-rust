// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
        LogLevel::Debug => debug(message),
    }
}
pub fn info(message: &str) -> String {
    String::from("[INFO]: ".to_owned() + message)
}
pub fn warn(message: &str) -> String {
    String::from("[WARNING]: ".to_owned() + message)
}
pub fn error(message: &str) -> String {
    String::from("[ERROR]: ".to_owned() + message)
}

pub fn debug(message: &str) -> String {
    String::from("[DEBUG]: ".to_owned() + message)
}
