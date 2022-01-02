use colored::Colorize;
use std::fmt;

/// The error message shown when the user tries to sort with source and/or target
/// directories that don't exist
#[derive(Clone, Debug)]
pub struct PathDoesNotExistError {
    pub path: String,
}
impl fmt::Display for PathDoesNotExistError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} \"{}\" does not exist.", format!("Error:").red(), format!("{}", self.path).bold())
    }
}
impl std::error::Error for PathDoesNotExistError { }